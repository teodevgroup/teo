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
use crate::parser::parser::parser::Parser;
use crate::prelude::{Graph, Object, Value};
use crate::seeder::data_set::{DataSet, Group, normalize_dataset_relations, Record};
use crate::teon;

pub(crate) async fn seed(action: SeedCommandAction, graph: &Graph, data_sets: &Vec<DataSet>, names: Vec<String>) {
    for name in &names {
        let data_set = data_sets.iter().find(|ds| &ds.name == name).unwrap();
        match action {
            SeedCommandAction::Seed => seed_dataset(graph, normalize_dataset_relations(data_set, graph)).await,
            SeedCommandAction::Unseed => unseed_dataset(graph, normalize_dataset_relations(data_set, graph)).await,
            SeedCommandAction::Reseed => reseed_dataset(graph, normalize_dataset_relations(data_set, graph)).await,
        }
    }
}

pub(crate) async fn seed_dataset(graph: &Graph, dataset: &DataSet) {
    let ordered_groups = ordered_group(&dataset.groups, graph);
    // // newly added records, we only update reference and relationships for these records.
    // let mut added_records: HashMap<String, Vec<String>> = hashmap!{};
    // // First, insert into database with required foreign key relations
    // for group in &ordered_groups {
    //     let group_model = graph.model(group.name.as_str()).unwrap();
    //     let mut added_names = vec![];
    //     for record in group.records.iter() {
    //         let seed_records: Vec<Object> = graph.find_many(seed_data_model.name(), &teon!({
    //             "where": {
    //                 "group": group.name.as_str(),
    //                 "dataset": dataset.name.as_str(),
    //             }
    //         })).await.unwrap();
    //         for seed_record in seed_records.iter() {
    //             let existing: Option<Object> = graph.find_unique(group_model.name(), &teon!({
    //                 "where": record_json_string_to_where_unique(seed_record.get_value("record").unwrap().as_str().unwrap(), group_model)
    //             })).await.unwrap();
    //             if existing.is_none() {
    //                 perform_insert_into_database(dataset, group, record, group_model, seed_data_model, graph).await;
    //                 added_names.push(record.name.clone());
    //             }
    //         }
    //     }
    //     added_records.insert(group.name.clone(), added_names);
    // }
    // // Second, setup optional relations and array relations
    // setup_relations(graph, dataset, &ordered_groups, seed_data_model, Some(&added_records)).await
}

pub(crate) async fn unseed_dataset(graph: &Graph, data_set: &DataSet) {
    let seed_data_model = graph.model("__TeoSeedData").unwrap();

}

pub(crate) async fn reseed_dataset(graph: &Graph, data_set: &DataSet) {
    let seed_data_model = graph.model("__TeoSeedData").unwrap();

}

async fn setup_relations(graph: &Graph, dataset: &DataSet, ordered_groups: &Vec<&Group>, seed_data_model: &Model, limit: Option<&HashMap<String, Vec<String>>>) {
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

/// This perform, saves an an object into the database. It doesn't setup relationships without
/// required foreign keys.
async fn perform_insert_into_database(dataset: &DataSet, group: &Group, record: &Record, group_model: &Model, seed_data_model: &Model, graph: &Graph) {
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
    let record_object = graph.new_object(seed_data_model.name(), Action::from_u32(PROGRAM_CODE), ActionSource::ProgramCode).unwrap();
    record_object.update_teon(&teon!({
        "group": group.name.as_str(),
        "dataset": dataset.name.as_str(),
        "record": object_identifier_in_json(&object),
    })).await.unwrap();
    record_object.save().await.unwrap();
}

fn record_json_string_to_where_unique(json_str: &str, model: &Model) -> Value {
    let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
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