use std::collections::HashMap;
use bson::oid::ObjectId;
use itertools::Itertools;
use maplit::hashmap;
use serde_json::json;
use crate::core::app::command::SeedCommandAction;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::model::Model;
use crate::core::relation::Relation;
use crate::core::result::Result;
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
            SeedCommandAction::Reseed => reseed_dataset(graph, normalize_dataset_relations(dataset, graph)).await,
            SeedCommandAction::Unseed => unseed_dataset(graph, normalize_dataset_relations(dataset, graph)).await,
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
            let existing = seed_records.iter().find(|r| &r.name() == &record.name).is_some();
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
                perform_remove_from_database(dataset, seed_record, group_model, graph).await;
            }
        }
    }
    // Second, setup optional relations and array relations
    setup_new_relations(graph, dataset, &ordered_groups, Some(&added_records)).await;
    // Last, remove records for user removed groups
    remove_records_for_user_removed_groups(dataset, &ordered_groups, graph).await;
}

async fn remove_records_for_user_removed_groups(dataset: &DataSet, ordered_groups: &Vec<&Group>, graph: &Graph) {
    let user_removed_seed_records_for_group = GroupRecord::find_many(teon!({
        "where": {
            "dataset": dataset.name.as_str(),
            "group": {
                "notIn": Value::Vec(ordered_groups.iter().map(|g| Value::String(g.name.clone())).collect()),
            },
        }
    })).await.unwrap();
    for record in user_removed_seed_records_for_group {
        perform_remove_from_database(dataset, &record, graph.model(record.group().as_str()).unwrap(), graph).await;
    }
}

pub(crate) async fn reseed_dataset(graph: &Graph, dataset: &DataSet) {
    let ordered_groups = ordered_group(&dataset.groups, graph);
    for group in &ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let seed_records = GroupRecord::find_many(teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        for record in group.records.iter() {
            if let Some(seed_record) = seed_records.iter().find(|r| &r.name() == &record.name) {
                // recreate or update
                perform_recreate_or_update_an_record(dataset, group, record, group_model, graph, seed_record).await;
            } else {
                // create
                perform_insert_into_database(dataset, group, record, group_model, graph).await;
            }
        }
        // delete records which are not recorded in user dataset
        for seed_record in seed_records.iter() {
            let existing = group.records.iter().find(|r| &r.name == &seed_record.name()).is_some();
            if !existing {
                perform_remove_from_database(dataset, seed_record, group_model, graph).await;
            }
        }
    }
    // Second, setup optional relations and array relations
    sync_relations(graph, dataset, &ordered_groups).await;
    // Last, remove records for user removed groups
    remove_records_for_user_removed_groups(dataset, &ordered_groups, graph).await;
}

pub(crate) async fn unseed_dataset(graph: &Graph, dataset: &DataSet) {
    let mut ordered_groups = ordered_group(&dataset.groups, graph);
    ordered_groups.reverse();
    for group in ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let seed_records = GroupRecord::find_many(teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        // delete records
        for seed_record in seed_records.iter() {
            perform_remove_from_database(dataset, seed_record, graph.model(seed_record.group().as_str()).unwrap(), graph).await;
        }
    }
}

async fn sync_relations(graph: &Graph, dataset: &DataSet, ordered_groups: &Vec<&Group>) {
    for group in ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let should_process = group_model.relations().iter().find(|r| !(r.has_foreign_key() && r.is_required())).is_some();
        if !should_process { continue }
        let seed_records = GroupRecord::find_many(teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        for record in group.records.iter() {
            let seed_record = seed_records.iter().find(|o| o.name().as_str() == &record.name).unwrap();
            let object: Object = graph.find_unique(group_model.name(), &teon!({
                "where": record_json_string_to_where_unique(seed_record.record().as_str(), group_model)
            })).await.unwrap();
            for relation in group_model.relations() {
                // find relations
                let relation_records = GroupRelation::find_many(teon!({
                    "where": {
                        "OR": [
                            {
                                "dataset": dataset.name.as_str(),
                                "groupA": object.model().name(),
                                "relationA": relation.name(),
                                "nameA": record.name.as_str(),
                                "groupB": relation.model(),
                            },
                            {
                                "dataset": dataset.name.as_str(),
                                "groupB": object.model().name(),
                                "relationB": relation.name(),
                                "nameB": record.name.as_str(),
                                "groupA": relation.model(),
                            }
                        ]
                    }
                })).await.unwrap();
                let mut relation_record_refs: Vec<&GroupRelation> = relation_records.iter().collect();
                if let Some(reference) = record.value.as_hashmap().unwrap().get(relation.name()) {
                    if let Some(references) = reference.as_vec() {
                        for reference in references {
                            sync_relation_internal(record, reference, relation, dataset, graph, &object, &relation_records, &mut relation_record_refs).await;
                        }
                    } else {
                        sync_relation_internal(record, reference, relation, dataset, graph, &object, &relation_records, &mut relation_record_refs).await;
                    }
                } else {
                    // find relations and cut
                    for relation_record in relation_record_refs {
                        cut_relation(relation_record, seed_record, graph, group_model, dataset, &object).await;
                    }
                }
            }
        }
    }
}

async fn setup_new_relations(graph: &Graph, dataset: &DataSet, ordered_groups: &Vec<&Group>, limit: Option<&HashMap<String, Vec<String>>>) {
    for group in ordered_groups {
        let group_model = graph.model(group.name.as_str()).unwrap();
        let should_process = group_model.relations().iter().find(|r| !(r.has_foreign_key() && r.is_required())).is_some();
        if !should_process { continue }
        let seed_records = GroupRecord::find_many(teon!({
            "where": {
                "group": group.name.as_str(),
                "dataset": dataset.name.as_str(),
            }
        })).await.unwrap();
        for record in group.records.iter() {
            if !(limit.is_none() || limit.unwrap().get(&group.name).unwrap().contains(&record.name)) { continue }
            let seed_record = seed_records.iter().find(|o| o.name().as_str() == &record.name).unwrap();
            let object: Object = graph.find_unique(group_model.name(), &teon!({
                "where": record_json_string_to_where_unique(seed_record.record().as_str(), group_model)
            })).await.unwrap();
            for relation in group_model.relations() {
                if let Some(reference) = record.value.as_hashmap().unwrap().get(relation.name()) {
                    if let Some(references) = reference.as_vec() {
                        for reference in references {
                            setup_relations_internal(record, reference, relation, dataset, graph, &object).await;
                        }
                    } else {
                        setup_relations_internal(record, reference, relation, dataset, graph, &object).await;
                    }
                }
            }
        }
    }
}

async fn sync_relation_internal(record: &Record, reference: &Value, relation: &Relation, dataset: &DataSet, graph: &Graph, object: &Object, relation_records: &Vec<GroupRelation>, relation_record_refs: &mut Vec<&GroupRelation>) {
    let that_name = reference.as_raw_enum_choice().unwrap();
    if let Some(existing_relation_record) = relation_records.iter().find(|r| {
        (&r.name_a() == record.name.as_str() && r.name_b() == that_name) ||
            (&r.name_b() == record.name.as_str() && r.name_a() == that_name)
    }) {
        let index = relation_record_refs.iter().position(|r| *r == existing_relation_record).unwrap();
        relation_record_refs.remove(index);
    }
    setup_relations_internal(record, reference, relation, dataset, graph, object).await;
}

async fn setup_relations_internal(record: &Record, reference: &Value, relation: &Relation, dataset: &DataSet, graph: &Graph, object: &Object) {
    let that_name = reference.as_raw_enum_choice().unwrap();
    let that_seed_record = GroupRecord::find_first(teon!({
        "where": {
            "group": relation.model(),
            "dataset": dataset.name.as_str(),
            "name": that_name,
        }
    })).await.unwrap();
    let that_object: Object = graph.find_unique(relation.model(), &teon!({
        "where": record_json_string_to_where_unique(that_seed_record.record(), graph.model(relation.model()).unwrap())
    })).await.unwrap();
    if relation.is_optional() && relation.has_foreign_key() {
        // update this record
        for (local, foreign) in relation.iter() {
            object.set_value(local, that_object.get_value(foreign).unwrap()).unwrap();
        }
        object.save_for_seed_without_required_relation().await.unwrap();
    } else if !relation.has_join_table() {
        // update that record
        for (local, foreign) in relation.iter() {
            that_object.set_value(foreign, object.get_value(local).unwrap()).unwrap();
        }
        that_object.save_for_seed_without_required_relation().await.unwrap();
    } else {
        let (through_model, through_relation) = graph.through_relation(relation);
        let (_, through_that_relation) = graph.through_opposite_relation(relation);
        let mut where_unique: HashMap<String, Value> = HashMap::new();
        for (local, foreign) in through_relation.iter() {
            where_unique.insert(local.to_string(), object.get_value(foreign).unwrap());
        }
        for (local, foreign) in through_that_relation.iter() {
            where_unique.insert(local.to_string(), that_object.get_value(foreign).unwrap());
        }
        let link_record: Result<Object> = graph.find_first(through_model.name(), &teon!({
            "where": Value::HashMap(where_unique.clone())
        })).await;
        if link_record.is_err() {
            let link_object = graph.create_object(through_model.name(), Value::HashMap(where_unique)).await.unwrap();
            link_object.save_for_seed_without_required_relation().await.unwrap();
        }
    }
    // update relation record
    let exist_relation_record = GroupRelation::find_first(teon!({
        "where": {
            "OR": [
                {
                    "dataset": dataset.name.as_str(),
                    "groupA": object.model().name(),
                    "relationA": relation.name(),
                    "nameA": record.name.as_str(),
                    "groupB": that_object.model().name(),
                    "nameB": that_name,
                },
                {
                    "dataset": dataset.name.as_str(),
                    "groupB": object.model().name(),
                    "relationB": relation.name(),
                    "nameB": record.name.as_str(),
                    "groupA": that_object.model().name(),
                    "nameA": that_name
                }
            ]
        }
    })).await;
    if exist_relation_record.is_err() {
        // not exist, create
        let that_relation = graph.opposite_relation(relation).1;
        let new_relation_record = GroupRelation::new(teon!({
            "dataset": dataset.name.as_str(),
            "groupA": object.model().name(),
            "relationA": relation.name(),
            "nameA": record.name.as_str(),
            "groupB": that_object.model().name(),
            "relationB": if that_relation.is_some() { Value::String(that_relation.unwrap().name().to_owned()) } else { Value::Null },
            "nameB": that_name,
        })).await;
        new_relation_record.save().await.unwrap();
    }
}

/// This perform, deletes an object from the databse.
async fn perform_remove_from_database(dataset: &DataSet, record: &GroupRecord, group_model: &Model, graph: &Graph) {
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
                    "groupA": record.group().as_str(),
                    "nameA": record.name().as_str(),
                },
                {
                    "dataset": dataset.name.as_str(),
                    "groupB": record.group().as_str(),
                    "nameB": record.name().as_str(),
                }
            ]
        }
    })).await.unwrap();
    for relation in relations {
        cut_relation(&relation, record, graph, group_model, dataset, &exist).await;
    }
    // Second, delete it and the seed record
    exist.delete().await.unwrap();
    record.delete().await.unwrap();
}

async fn cut_relation(relation: &GroupRelation, record: &GroupRecord, graph: &Graph, group_model: &Model, dataset: &DataSet, exist: &Object) {
    let rel_name = if record.group().as_str() == relation.group_a() { relation.relation_a() } else { relation.relation_b() };
    let model_relation = group_model.relation(&rel_name).unwrap();
    if model_relation.has_foreign_key() {
        // If has foreign keys, this relation is already cut
        relation.delete().await.unwrap();
        return
    }
    // get that record
    let that_model_name = if record.group().as_str() == relation.group_a() { relation.group_b() } else { relation.group_a() };
    let that_model = graph.model(&that_model_name).unwrap();
    let that_name = if record.group().as_str() == relation.group_a() { relation.name_b() } else { relation.name_a() };
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
        return
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
            return
        }
        let link_record = link_record.unwrap();
        link_record.delete().await.unwrap();
    } else {
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
            that_record.save_for_seed_without_required_relation().await.unwrap();
        }
    }
    relation.delete().await.unwrap();
}

async fn perform_recreate_or_update_an_record(dataset: &DataSet, group: &Group, record: &Record, group_model: &Model, graph: &Graph, seed_record: &GroupRecord) {
    let object: Result<Object> = graph.find_unique(group_model.name(), &teon!({
        "where": record_json_string_to_where_unique(seed_record.record(), group_model)
    })).await;
    if object.is_err() {
        seed_record.delete().await.unwrap();
        perform_insert_into_database(dataset, group, record, group_model, graph).await;
        return
    }
    let object = object.unwrap();
    let input = insert_or_update_input(dataset, group, record, group_model, graph).await;
    object.set_teon(&input).await.unwrap();
    object.save_for_seed_without_required_relation().await.unwrap();
    seed_record.set_record(object_identifier_in_json(&object));
    seed_record.save().await.unwrap();
}

async fn insert_or_update_input(dataset: &DataSet, group: &Group, record: &Record, group_model: &Model, graph: &Graph) -> Value {
    let mut input = teon!({});
    for (k, v) in record.value.as_hashmap().unwrap() {
        if group_model.field(k).is_some() {
            input.as_hashmap_mut().unwrap().insert(k.to_owned(), v.clone());
        } else if let Some(relation) = group_model.relation(k) {
            if relation.is_required() && relation.has_foreign_key() {
                // setup required relationship
                let that_record_name = v.as_raw_enum_choice().unwrap();
                let that_record_data = GroupRecord::find_first(teon!({
                    "where": {
                        "group": relation.model(),
                        "dataset": dataset.name.as_str(),
                        "name": that_record_name,
                    }
                })).await.unwrap();
                let that_record_identifier_json = that_record_data.record();
                let that_record: Object = graph.find_unique(relation.model(), &teon!({
                    "where": record_json_string_to_where_unique(&that_record_identifier_json, graph.model(relation.model()).unwrap())
                })).await.unwrap();
                for (field, reference) in relation.iter() {
                    input.as_hashmap_mut().unwrap().insert(field.to_owned(), that_record.get_value(reference).unwrap());
                }
                // update relation record
                let (_, opposite_relation) = graph.opposite_relation(relation);
                let relation_record = GroupRelation::new(teon!({
                    "dataset": dataset.name.as_str(),
                    "groupA": group.name.as_str(),
                    "relationA": relation.name(),
                    "nameA": record.name.as_str(),
                    "groupB": that_record.model().name(),
                    "relationB": if opposite_relation.is_some() { Value::String(opposite_relation.unwrap().name().to_owned()) } else { Value::Null },
                    "nameB": v.as_raw_enum_choice().unwrap()
                })).await;
                relation_record.save().await.unwrap();
            }
        }
    }
    input
}

/// This perform, saves an object into the database. It doesn't setup relationships without
/// required foreign keys.
async fn perform_insert_into_database(dataset: &DataSet, group: &Group, record: &Record, group_model: &Model, graph: &Graph) {
    let input = insert_or_update_input(dataset, group, record, group_model, graph).await;
    let object = graph.create_object(group_model.name(), &input).await.unwrap();
    object.save_for_seed_without_required_relation().await.unwrap();
    let record_object = GroupRecord::new(teon!({
        "group": group.name.as_str(),
        "dataset": dataset.name.as_str(),
        "name": record.name.as_str(),
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
    result.to_string()
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