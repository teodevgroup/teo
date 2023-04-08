use std::collections::HashMap;
use std::ops::Index;
use bson::oid::ObjectId;
use itertools::Itertools;
use maplit::hashmap;
use serde_json::json;
use crate::core::action::{Action, PROGRAM_CODE};
use crate::core::action::source::ActionSource;
use crate::core::app::command::SeedCommandAction;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::model::Model;
use crate::core::result::Result;
use crate::parser::parser::parser::Parser;
use crate::prelude::{Graph, Object, Value};
use crate::seeder::data_set::{DataSet, Group, normalize_dataset_relations, Record};
use crate::seeder::models::group_record::GroupRecord;
use crate::seeder::models::group_relation::GroupRelation;
use crate::teon;

pub(crate) async fn seed(action: SeedCommandAction, graph: &Graph, datasets: &Vec<DataSet>, names: Vec<String>) {
    // seed for user
    for name in &names {
        let dataset = datasets.iter().find(|ds| &ds.name == name).unwrap();
        match action {
            SeedCommandAction::Seed => seed_dataset(graph, normalize_dataset_relations(dataset, graph)).await,
            SeedCommandAction::Unseed => unseed_dataset(graph, normalize_dataset_relations(dataset, graph)).await,
            SeedCommandAction::Reseed => reseed_dataset(graph, normalize_dataset_relations(dataset, graph)).await,
        }
    }
    remove_user_deleted_dataset_records_and_relations(datasets).await;
}

pub(crate) async fn seed_dataset(graph: &Graph, dataset: &DataSet) {
    let ordered_groups = ordered_group(&dataset.groups, graph);
    // newly added records, we only update reference and relationships for these records.
    let mut added_records: HashMap<String, Vec<String>> = hashmap!{};
    // First, insert into database with required foreign key relations
    for group in &ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let mut added_names = vec![];
        let seed_records = GroupRecord::find_many(teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        for record in group.records.iter() {
            let mut existing = false;
            for seed_record in seed_records.iter() {
                if &seed_record.name() == &record.name {
                    existing = true;
                }
            }
            if !existing {
                perform_insert_into_database(dataset, group, record, group_model, graph).await;
                added_names.push(record.name.clone());
            }
        }
        added_records.insert(group.name.clone(), added_names);
        // delete records which are not recorded in user dataset
        for seed_record in seed_records.iter() {
            let existing = group.records.iter().find(|r| &r.name == &seed_record.name()).is_some();
            if !existing {
                perform_remove_from_database(dataset, group, seed_record, group_model, graph).await;
            }
        }
    }
    // Second, setup optional relations and array relations
    setup_relations(graph, dataset, &ordered_groups, Some(&added_records)).await;
}

pub(crate) async fn unseed_dataset(graph: &Graph, data_set: &DataSet) {
    let seed_data_model = graph.model("__TeoSeedData").unwrap();

}

pub(crate) async fn reseed_dataset(graph: &Graph, data_set: &DataSet) {
    let seed_data_model = graph.model("__TeoSeedData").unwrap();

}

async fn setup_relations(graph: &Graph, dataset: &DataSet, ordered_groups: &Vec<&Group>, limit: Option<&HashMap<String, Vec<String>>>) {
    for group in ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let should_process = group_model.relations().iter().find(|r| !(r.has_foreign_key() && r.is_required())).is_some();
        if !should_process { continue }
        let seed_records: Vec<Object> = graph.find_many(seed_data_model.name(), &teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        for record in group.records.iter() {
            if !(limit.is_none() || limit.unwrap().get(&group.name).unwrap().contains(&record.name)) { continue }
            let seed_record = seed_records.iter().find(|o| o.get_value("name").unwrap().as_str().unwrap() == &record.name).unwrap();
            let object: Object = graph.find_unique(group_model.name(), &teon!({
                "where": record_json_string_to_where_unique(seed_record.get_value("record").unwrap().as_str().unwrap(), group_model)
            })).await.unwrap();
            for relation in group_model.relations() {
                if relation.is_optional() && relation.has_foreign_key() {
                    // update this record

                } else if relation.has_join_table() {
                    // create link records
                } else if !relation.has_foreign_key() {
                    // update record on that side
                }

            }

        }
    }
}

/// This perform, deletes an object from the databse.
async fn perform_remove_from_database(dataset: &DataSet, group: &Group, record: &GroupRecord, group_model: &Model, graph: &Graph) {
    let json_identifier = record.record();
    let exist: Result<Object> = graph.find_unique(group_model.name(), &teon!({
        "where": record_json_string_to_where_unique(json_identifier, group_model)
    })).await;
    if exist.is_err() {
        // This record doesn't exist, cannot delete it or cut its relationships
        record.delete().await.unwrap();
        return
    }
    // First, cut relations
    let exist = exist.unwrap();
    let relations = GroupRelation::find_many(teon!({
        "where": {
            "OR": [
                {
                    "dataset": dataset.name.as_str(),
                    "groupA": group.name.as_str(),
                    "nameA": record.name().as_str(),
                },
                {
                    "dataset": dataset.name.as_str(),
                    "groupB": group.name.as_str(),
                    "nameB": record.name().as_str(),
                }
            ]
        }
    })).await.unwrap();
    for relation in relations {
        let rel_name = if group.name.as_str() == relation.group_a() { relation.relation_a() } else { relation.relation_b() };
        let model_relation = group_model.relation(&rel_name).unwrap();
        if model_relation.has_foreign_key() {
            // If has foreign keys, this relation is already cut
            relation.delete().await.unwrap();
            continue
        }
        // get that record
        let that_model_name = if group.name.as_str() == relation.group_a() { relation.group_b() } else { relation.group_a() };
        let that_model = graph.model(&that_model_name).unwrap();
        let that_name = if group.name.as_str() == relation.group_a() { relation.name_b() } else { relation.name_a() };
        let seed_record = GroupRecord::find_first(teon!({
                "dataset": dataset.name.as_str(),
                "group": that_model_name.as_str(),
                "name": that_name.as_str()
            })).await.unwrap();
        let identifier = seed_record.record();
        let unique = record_json_string_to_where_unique(&identifier, that_model);
        let that_record: Result<Object> = graph.find_unique(that_model_name.as_str(), &teon!({
                "where": unique
            })).await;
        if that_record.is_err() {
            relation.delete().await.unwrap();
            continue
        }
        let that_record = that_record.unwrap();
        if model_relation.has_join_table() {
            let (through_model, through_relation) = graph.through_relation(model_relation);
            let (_, through_that_relation) = graph.through_opposite_relation(model_relation);
            let mut where_unique: HashMap<String, Value> = HashMap::new();
            for (local, foreign) in through_relation.iter() {
                where_unique.insert(local.to_string(), exist.get_value(foreign).unwrap());
            }
            for (local, foreign) in through_that_relation.iter() {
                where_unique.insert(local.to_string(), that_record.get_value(foreign).unwrap());
            }
            let link_record: Result<Object> = graph.find_first(through_model.name(), &teon!({
                "where": Value::HashMap(where_unique)
            })).await;
            if link_record.is_err() {
                // Maybe this record is deleted already
                relation.delete().await.unwrap();
                continue
            }
            let link_record = link_record.unwrap();
            link_record.delete().await.unwrap();
        } else {
            let that_record = that_record.unwrap();
            let mut link_to_self = true;
            for (local, foreign) in model_relation.iter() {
                if that_record.get_value(foreign).unwrap() != exist.get_value(local).unwrap() {
                    link_to_self = false;
                }
            }
            if link_to_self {
                // nullify
                for (_local, foreign) in model_relation.iter() {
                    that_record.set_value(foreign, Value::Null).unwrap();
                }
                that_record.save().await.unwrap();
            }
        }
        relation.delete().await.unwrap();
    }
    // Second, delete it and the seed record
    exist.unwrap().delete().await.unwrap();
    record.delete().await.unwrap();
}

/// This perform, saves an object into the database. It doesn't setup relationships without
/// required foreign keys.
async fn perform_insert_into_database(dataset: &DataSet, group: &Group, record: &Record, group_model: &Model, graph: &Graph) {
    let object = graph.new_object(group_model.name(), Action::from_u32(PROGRAM_CODE), ActionSource::ProgramCode).unwrap();
    object.set_teon(&teon!({})).await.unwrap();
    let mut input = teon!({});
    for (k, v) in record.value.as_hashmap().unwrap() {
        if group_model.field(k).is_some() {
            input.as_hashmap_mut().unwrap().insert(k.to_owned(), v.clone());
        } else if let Some(relation) = group_model.relation(k) {
            if relation.is_required() && relation.has_foreign_key() {
                // setup required relationship
                let that_record_name = v.as_raw_enum_choice().unwrap();
                let that_record_data: Object = graph.find_first::<Object>(seed_data_model.name(), &teon!({
                    "where": {
                        "group": relation.model(),
                        "dataset": dataset.name.as_str(),
                        "name": that_record_name,
                    }
                })).await.unwrap();
                let that_record_identifier_json = that_record_data.get_value("record").unwrap().as_str().unwrap().to_string();
                let that_record: Object = graph.find_unique(relation.model(), &teon!({
                    "where": record_json_string_to_where_unique(&that_record_identifier_json, graph.model(relation.model()).unwrap())
                })).await.unwrap();
                for (field, reference) in relation.iter() {
                    input.as_hashmap_mut().unwrap().insert(field.to_owned(), that_record.get_value(reference).unwrap());
                }
            }
        }
    }
    object.update_teon(&input).await.unwrap();
    object.save().await.unwrap();
    let record_object = GroupRecord::new(teon!({
        "group": group.name.as_str(),
        "dataset": dataset.name.as_str(),
        "record": object_identifier_in_json(&object),
    })).await;
    record_object.save().await.unwrap();
}

fn record_json_string_to_where_unique(json_str: impl AsRef<str>, model: &Model) -> Value {
    let json_value: serde_json::Value = serde_json::from_str(json_str.as_ref()).unwrap();
    let json_object = json_value.as_object().unwrap();
    let mut result_teon_value = teon!({});
    for (k, v) in json_object {
        match model.field(k).unwrap().field_type() {
            FieldType::String => result_teon_value.as_hashmap_mut().unwrap().insert(k.to_owned(), Value::String(v.as_str().unwrap().to_string())),
            FieldType::ObjectId => result_teon_value.as_hashmap_mut().unwrap().insert(k.to_owned(), Value::ObjectId(ObjectId::parse_str(v.as_str().unwrap()).unwrap())),
            FieldType::I32 => result_teon_value.as_hashmap_mut().unwrap().insert(k.to_owned(), Value::I32(v.as_i64().unwrap() as i32)),
            FieldType::I64 => result_teon_value.as_hashmap_mut().unwrap().insert(k.to_owned(), Value::I64(v.as_i64().unwrap())),
            _ => unreachable!()
        };
    }
    result_teon_value
}

fn object_identifier_in_json(object: &Object) -> String {
    let identifier = object.identifier();
    let mut result = json!({});
    for (k, v) in identifier.as_hashmap().unwrap() {
        match object.model().field(k).unwrap().field_type() {
            FieldType::ObjectId => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::String(v.as_object_id().unwrap().to_string())),
            FieldType::String => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::String(v.as_str().unwrap().to_string())),
            FieldType::I32 => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::Number(serde_json::Number::from(v.as_i32().unwrap()))),
            FieldType::I64 => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::Number(serde_json::Number::from(v.as_i64().unwrap()))),
            _ => unreachable!(),
        };
    }
    result.as_str().unwrap().to_owned()
}

fn ordered_group<'a>(groups: &'a Vec<Group>, graph: &Graph) -> Vec<&'a Group> {
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for group in groups {
        let model_name = &group.name;
        let model = graph.model(model_name).unwrap();
        let mut model_deps = vec![];
        for relation in model.relations() {
            if relation.has_foreign_key() && relation.is_required() {
                model_deps.push(relation.model().to_string());
            }
        }
        deps.insert(model_name.to_owned(), model_deps);
    }
    let mut result = vec![];
    loop {
        if deps.is_empty() {
            break;
        }
        let mut has_some = false;
        for (model_name, model_deps) in deps.iter() {
            if model_deps.is_empty() {
                result.push(groups.iter().find(|g| &g.name == model_name).unwrap());
                has_some = true;
            }
        }
        if !has_some {
            panic!("Circular required relationship between these models: `{}'.", deps.keys().join(","))
        }
        for group in &result {
            deps.remove(&group.name);
        }
        for (model_name, model_deps) in deps.iter_mut() {
            for group in &result {
                if let Some(index) = model_deps.iter().position(|x| x == &group.name) {
                    model_deps.remove(index);
                }
            }
        }
    }
    result
}

async fn remove_user_deleted_dataset_records_and_relations(datasets: &Vec<DataSet>) {
    // remove seed data set records if user removed some seed data set
    let names = Value::Vec(datasets.iter().map(|d| Value::String(d.name.clone())).collect::<Vec<Value>>());
    let records_to_remove = GroupRecord::find_many(teon!({
        "where": {
            "dataset": {
                "notIn": &names,
            }
        }
    })).await.unwrap();
    for record in records_to_remove {
        record.delete().await.unwrap();
    }
    let relations_to_remove = GroupRelation::find_many(teon!({
        "where": {
            "dataset": {
                "notIn": names,
            }
        }
    })).await.unwrap();
    for relation in relations_to_remove {
        relation.delete().await.unwrap();
    }
}