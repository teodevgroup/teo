use bson::oid::ObjectId;
use indexmap::{IndexMap, indexmap};
use itertools::Itertools;
use key_path::path;
use serde_json::json;
use teo_parser::r#type::Type;
use crate::prelude::Value;
use crate::seeder::models::data_set_record::DataSetRecord;
use crate::seeder::models::data_set_relation::DataSetRelation;
use teo_runtime::teon;
use crate::cli::command::SeedCommandAction;
use teo_result::Result;
use teo_runtime::connection::transaction;
use teo_runtime::data_set::{DataSet, Group, Record};
use teo_runtime::model::field::is_optional::IsOptional;
use teo_runtime::model::{Model, Object, Relation};
use teo_runtime::model::field::typed::Typed;
use teo_runtime::traits::named::Named;

pub(crate) async fn seed(action: SeedCommandAction, datasets: Vec<DataSet>, ctx: transaction::Ctx, exit: bool) -> Result<()> {
    // seed for user
    for dataset in &datasets {
        match action {
            SeedCommandAction::Seed => seed_dataset(dataset, ctx.clone()).await,
            SeedCommandAction::Reseed => reseed_dataset(dataset, ctx.clone()).await,
            SeedCommandAction::Unseed => unseed_dataset(dataset, ctx.clone()).await,
        }
    }
    remove_user_deleted_dataset_records_and_relations(&datasets, ctx).await;
    if exit {
        std::process::exit(0);
    } else {
        Ok(())
    }
}

pub(crate) async fn seed_dataset(dataset: &DataSet, ctx: transaction::Ctx) {
    let ordered_groups = ordered_group(&dataset.groups, ctx.clone());
    // newly added records, we only update reference and relationships for these records.
    let mut added_records: IndexMap<String, Vec<String>> = indexmap!{};
    // First, insert into database with required foreign key relations
    for group in &ordered_groups {
        let group_model = ctx.namespace().model_at_path(&group.model_path()).unwrap();
        let mut added_names = vec![];
        let seed_records = DataSetRecord::find_many(teon!({
            "where": {
                "group": group.name.join(".").as_str(),
                "dataSet": dataset.name.join(".").as_str(),
            }
        }), ctx.clone()).await.unwrap();
        for record in group.records.iter() {
            let existing = seed_records.iter().find(|r| &r.name() == &record.name).is_some();
            if !existing {
                perform_insert_into_database(dataset, group, record, group_model, ctx.clone()).await;
                added_names.push(record.name.clone());
            }
        }
        added_records.insert(group.name.join("."), added_names);
        // delete records which are not recorded in user dataset
        for seed_record in seed_records.iter() {
            let existing = group.records.iter().find(|r| &r.name == &seed_record.name()).is_some();
            if !existing {
                perform_remove_from_database(dataset, seed_record, group_model, ctx.clone()).await;
            }
        }
    }
    // Second, setup optional relations and array relations
    setup_new_relations(dataset, &ordered_groups, Some(&added_records), ctx.clone()).await;
    // Last, remove records for user removed groups
    remove_records_for_user_removed_groups(dataset, &ordered_groups, ctx.clone()).await;
}

async fn remove_records_for_user_removed_groups(dataset: &DataSet, ordered_groups: &Vec<&Group>, ctx: transaction::Ctx) {
    let user_removed_seed_records_for_group = DataSetRecord::find_many(teon!({
        "where": {
            "dataSet": dataset.name.join(".").as_str(),
            "group": {
                "notIn": Value::Array(ordered_groups.iter().map(|g| Value::String(g.name.join("."))).collect()),
            },
        }
    }), ctx.clone()).await.unwrap();
    for record in user_removed_seed_records_for_group {
        let model = ctx.namespace().model_at_path(&record.group());
        if model.is_some() {
            perform_remove_from_database(dataset, &record, model.unwrap(), ctx.clone()).await;
        } else {
            // this table is already dropped
            record.delete().await.unwrap();
        }
    }
    let user_removed_seed_relations_for_group = DataSetRelation::find_many(teon!({
        "where": {
            "OR": [
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupA": {
                        "notIn": Value::Array(ordered_groups.iter().map(|g| Value::String(g.name.join("."))).collect())
                    },
                },
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupB": {
                        "notIn": Value::Array(ordered_groups.iter().map(|g| Value::String(g.name.join("."))).collect()),
                    }
                }
            ]
        }
    }), ctx.clone()).await.unwrap();
    for relation in user_removed_seed_relations_for_group {
        let group_a_string = relation.group_a();
        let group_b_string = relation.group_b();
        let group_a: Vec<String> = group_a_string.split(".").map(ToOwned::to_owned).collect();
        let group_b: Vec<String> = group_b_string.split(".").map(ToOwned::to_owned).collect();
        let model_a = ctx.namespace().model_at_path(&group_a);
        let model_b = ctx.namespace().model_at_path(&group_b);
        if model_a.is_none() || model_b.is_none() {
            relation.delete().await.unwrap();
        }
    }
}

pub(crate) async fn reseed_dataset(dataset: &DataSet, ctx: transaction::Ctx) {
    let ordered_groups = ordered_group(&dataset.groups, ctx.clone());
    for group in &ordered_groups {
        let group_model = ctx.namespace().model_at_path(&group.model_path()).unwrap();
        let seed_records = DataSetRecord::find_many(teon!({
            "where": {
                "group": group.name.join(".").as_str(),
                "dataSet": dataset.name.join(".").as_str(),
            }
        }), ctx.clone()).await.unwrap();
        for record in group.records.iter() {
            if let Some(seed_record) = seed_records.iter().find(|r| &r.name() == &record.name) {
                // recreate or update
                perform_recreate_or_update_an_record(dataset, group, record, group_model, seed_record, ctx.clone()).await;
            } else {
                // create
                perform_insert_into_database(dataset, group, record, group_model, ctx.clone()).await;
            }
        }
        // delete records which are not recorded in user dataset
        for seed_record in seed_records.iter() {
            let existing = group.records.iter().find(|r| &r.name == &seed_record.name()).is_some();
            if !existing {
                perform_remove_from_database(dataset, seed_record, group_model, ctx.clone()).await;
            }
        }
    }
    // Second, setup optional relations and array relations
    sync_relations(dataset, &ordered_groups, ctx.clone()).await;
    // Last, remove records for user removed groups
    remove_records_for_user_removed_groups(dataset, &ordered_groups, ctx.clone()).await;
}

pub(crate) async fn unseed_dataset(dataset: &DataSet, ctx: transaction::Ctx) {
    let mut ordered_groups = ordered_group(&dataset.groups, ctx.clone());
    ordered_groups.reverse();
    for group in ordered_groups {
        let seed_records = DataSetRecord::find_many(teon!({
            "where": {
                "group": group.name.join(".").as_str(),
                "dataSet": dataset.name.join(".").as_str(),
            }
        }), ctx.clone()).await.unwrap();
        // delete records
        for seed_record in seed_records.iter() {
            let model = ctx.namespace().model_at_path(&seed_record.group()).unwrap();
            perform_remove_from_database(dataset, seed_record, model, ctx.clone()).await;
        }
    }
}

async fn sync_relations(dataset: &DataSet, ordered_groups: &Vec<&Group>, ctx: transaction::Ctx) {
    for group in ordered_groups {
        let group_model = ctx.namespace().model_at_path(&group.model_path()).unwrap();
        let should_process = group_model.relations().values().find(|r| !(r.has_foreign_key() && r.is_required())).is_some();
        if !should_process { continue }
        let seed_records = DataSetRecord::find_many(teon!({
            "where": {
                "group": group.name.join(".").as_str(),
                "dataSet": dataset.name.join(".").as_str(),
            }
        }), ctx.clone()).await.unwrap();
        for record in group.records.iter() {
            let seed_record = seed_records.iter().find(|o| o.name().as_str() == &record.name).unwrap();
            let object: Object = ctx.find_unique(group_model, &teon!({
                "where": record_json_string_to_where_unique(seed_record.record().as_str(), group_model)
            }), None, path![]).await.unwrap().unwrap();
            for relation in group_model.relations().values() {
                // find relations
                let relation_records = DataSetRelation::find_many(teon!({
                    "where": {
                        "OR": [
                            {
                                "dataSet": dataset.name.join(".").as_str(),
                                "groupA": object.model().path().join("."),
                                "relationA": relation.name(),
                                "nameA": record.name.as_str(),
                                "groupB": relation.model_path().join("."),
                            },
                            {
                                "dataSet": dataset.name.join(".").as_str(),
                                "groupB": object.model().path().join("."),
                                "relationB": relation.name(),
                                "nameB": record.name.as_str(),
                                "groupA": relation.model_path().join("."),
                            }
                        ]
                    }
                }), ctx.clone()).await.unwrap();
                let mut relation_record_refs: Vec<&DataSetRelation> = relation_records.iter().collect();
                if let Some(reference) = record.value.as_dictionary().unwrap().get(relation.name()) {
                    if let Some(references) = reference.as_array() {
                        for reference in references {
                            sync_relation_internal(record, reference, relation, dataset, &object, &relation_records, &mut relation_record_refs, ctx.clone()).await;
                        }
                    } else {
                        sync_relation_internal(record, reference, relation, dataset, &object, &relation_records, &mut relation_record_refs, ctx.clone()).await;
                    }
                } else {
                    // find relations and cut
                    for relation_record in relation_record_refs {
                        cut_relation(relation_record, seed_record, group_model, dataset, &object, ctx.clone()).await;
                    }
                }
            }
        }
    }
}

async fn setup_new_relations(dataset: &DataSet, ordered_groups: &Vec<&Group>, limit: Option<&IndexMap<String, Vec<String>>>, ctx: transaction::Ctx) {
    for group in ordered_groups {
        let group_model = ctx.namespace().model_at_path(&group.model_path()).unwrap();
        let should_process = group_model.relations().values().find(|r| !(r.has_foreign_key() && r.is_required())).is_some();
        if !should_process { continue }
        let seed_records = DataSetRecord::find_many(teon!({
            "where": {
                "group": group.name.join(".").as_str(),
                "dataSet": dataset.name.join(".").as_str(),
            }
        }), ctx.clone()).await.unwrap();
        for record in group.records.iter() {
            if !(limit.is_none() || limit.unwrap().get(&group.name.join(".")).unwrap().contains(&record.name)) { continue }
            let seed_record = seed_records.iter().find(|o| o.name().as_str() == &record.name).unwrap();
            let object: Object = ctx.find_unique(group_model, &teon!({
                "where": record_json_string_to_where_unique(seed_record.record().as_str(), group_model)
            }), None, path![]).await.unwrap().unwrap();
            for relation in group_model.relations().values() {
                if let Some(reference) = record.value.as_dictionary().unwrap().get(relation.name()) {
                    if let Some(references) = reference.as_array() {
                        for reference in references {
                            setup_relations_internal(record, reference, relation, dataset, &object, ctx.clone()).await;
                        }
                    } else {
                        setup_relations_internal(record, reference, relation, dataset, &object, ctx.clone()).await;
                    }
                }
            }
        }
    }
}

async fn sync_relation_internal<'a>(record: &Record, reference: &'a Value, relation: &'static Relation, dataset: &DataSet, object: &'a Object, relation_records: &'a Vec<DataSetRelation>, relation_record_refs: &mut Vec<&'a DataSetRelation>, ctx: transaction::Ctx) {
    let that_name = reference.as_str().unwrap().to_string();
    if let Some(existing_relation_record) = relation_records.iter().find(|r| {
        (&r.name_a() == record.name.as_str() && r.name_b() == that_name) ||
            (&r.name_b() == record.name.as_str() && r.name_a() == that_name)
    }) {
        let index = relation_record_refs.iter().position(|r| *r == existing_relation_record).unwrap();
        relation_record_refs.remove(index);
    }
    setup_relations_internal(record, reference, relation, dataset, object, ctx.clone()).await;
}

async fn setup_relations_internal<'a>(record: &Record, reference: &'a Value, relation: &'static Relation, dataset: &DataSet, object: &'a Object, ctx: transaction::Ctx) {
    let that_name = reference.as_str().unwrap().to_string();
    let that_seed_record = DataSetRecord::find_first(teon!({
        "where": {
            "group": relation.model_path().join("."),
            "dataSet": dataset.name.join(".").as_str(),
            "name": that_name.clone(),
        }
    }), ctx.clone()).await.unwrap().unwrap();
    let that_object: Object = ctx.find_unique(ctx.namespace().model_at_path(&relation.model_path()).unwrap(), &teon!({
        "where": record_json_string_to_where_unique(that_seed_record.record(), ctx.namespace().model_at_path(&relation.model_path()).unwrap())
    }), None, path![]).await.unwrap().unwrap();
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
        let (through_model, through_relation) = ctx.namespace().through_relation(relation);
        let (_, through_that_relation) = ctx.namespace().through_opposite_relation(relation);
        let mut where_unique: IndexMap<String, Value> = IndexMap::new();
        for (local, foreign) in through_relation.iter() {
            where_unique.insert(local.to_string(), object.get_value(foreign).unwrap());
        }
        for (local, foreign) in through_that_relation.iter() {
            where_unique.insert(local.to_string(), that_object.get_value(foreign).unwrap());
        }
        let link_record: Option<Object> = ctx.find_first(through_model, &teon!({
            "where": Value::Dictionary(where_unique.clone())
        }), None, path![]).await.unwrap();
        if link_record.is_none() {
            let link_object = ctx.create_object(through_model, Value::Dictionary(where_unique), None).await.unwrap();
            link_object.save_for_seed_without_required_relation().await.unwrap();
        }
    }
    // update relation record
    let exist_relation_record = DataSetRelation::find_first(teon!({
        "where": {
            "OR": [
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupA": object.model().path().join("."),
                    "relationA": relation.name(),
                    "nameA": record.name.as_str(),
                    "groupB": that_object.model().path().join("."),
                    "nameB": that_name.clone(),
                },
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupB": object.model().path().join("."),
                    "relationB": relation.name(),
                    "nameB": record.name.as_str(),
                    "groupA": that_object.model().path().join("."),
                    "nameA": that_name.clone()
                }
            ]
        }
    }), ctx.clone()).await.unwrap();
    if exist_relation_record.is_none() {
        // not exist, create
        let that_relation = ctx.namespace().opposite_relation(relation).1;
        let new_relation_record = DataSetRelation::new(teon!({
            "dataSet": dataset.name.join(".").as_str(),
            "groupA": object.model().path().join("."),
            "relationA": relation.name(),
            "nameA": record.name.as_str(),
            "groupB": that_object.model().path().join("."),
            "relationB": if that_relation.is_some() { Value::String(that_relation.unwrap().name().to_owned()) } else { Value::Null },
            "nameB": that_name.clone(),
        }), ctx.clone()).await.unwrap();
        new_relation_record.save().await.unwrap();
    }
}

/// This perform, deletes an object from the database.
async fn perform_remove_from_database<'a>(dataset: &DataSet, record: &'a DataSetRecord, group_model: &'static Model, ctx: transaction::Ctx) {
    let json_identifier = record.record();
    let exist: Option<Object> = ctx.find_unique(group_model, &teon!({
        "where": record_json_string_to_where_unique(json_identifier, group_model)
    }), None, path![]).await.unwrap();
    if exist.is_none() {
        // This record doesn't exist, cannot delete it or cut its relationships
        record.delete().await.unwrap();
        return
    }
    // First, cut relations
    let exist = exist.unwrap();
    let relations = DataSetRelation::find_many(teon!({
        "where": {
            "OR": [
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupA": record.group().join(".").as_str(),
                    "nameA": record.name().as_str(),
                },
                {
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupB": record.group().join(".").as_str(),
                    "nameB": record.name().as_str(),
                }
            ]
        }
    }), ctx.clone()).await.unwrap();
    for relation in relations {
        cut_relation(&relation, record, group_model, dataset, &exist, ctx.clone()).await;
    }
    // Second, delete it and the seed record
    exist.delete().await.unwrap();
    record.delete().await.unwrap();
}

async fn cut_relation<'a>(relation: &'a DataSetRelation, record: &'a DataSetRecord, group_model: &'static Model, dataset: &DataSet, exist: &'a Object, ctx: transaction::Ctx) {
    let rel_name = if record.group().join(".").as_str() == relation.group_a() { relation.relation_a() } else { relation.relation_b() };
    let model_relation = group_model.relation(&rel_name).unwrap();
    if model_relation.has_foreign_key() {
        // If has foreign keys, this relation is already cut
        relation.delete().await.unwrap();
        return
    }
    // get that record
    let that_model_name = if record.group().join(".").as_str() == relation.group_a() { relation.group_b() } else { relation.group_a() };
    let that_model_path: Vec<String> = that_model_name.split(".").map(|s| s.to_string()).collect();
    let that_model = ctx.namespace().model_at_path(&that_model_path).unwrap();
    let that_name = if record.group().join(".").as_str() == relation.group_a() { relation.name_b() } else { relation.name_a() };
    let that_record_record = DataSetRecord::find_first(teon!({
        "where": {
            "dataSet": dataset.name.join(".").as_str(),
            "group": that_model_name.as_str(),
            "name": that_name.as_str()
        }
    }), ctx.clone()).await.unwrap().unwrap();
    let identifier = that_record_record.record();
    let that_record_where_unique = record_json_string_to_where_unique(&identifier, that_model);
    let that_record: Option<Object> = ctx.find_unique(that_model, &teon!({
            "where": that_record_where_unique
        }), None, path![]).await.unwrap();
    if that_record.is_none() {
        relation.delete().await.unwrap();
        return
    }
    let that_record = that_record.unwrap();
    if model_relation.has_join_table() {
        let (through_model, through_relation) = ctx.namespace().through_relation(model_relation);
        let (_, through_that_relation) = ctx.namespace().through_opposite_relation(model_relation);
        let mut where_unique: IndexMap<String, Value> = IndexMap::new();
        for (local, foreign) in through_relation.iter() {
            where_unique.insert(local.to_string(), exist.get_value(foreign).unwrap());
        }
        for (local, foreign) in through_that_relation.iter() {
            where_unique.insert(local.to_string(), that_record.get_value(foreign).unwrap());
        }
        let link_record: Option<Object> = ctx.find_first(through_model, &teon!({
            "where": Value::Dictionary(where_unique)
        }), None, path![]).await.unwrap();
        if link_record.is_none() {
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

async fn perform_recreate_or_update_an_record<'a>(dataset: &DataSet, group: &Group, record: &Record, group_model: &'static Model, seed_record: &'a DataSetRecord, ctx: transaction::Ctx) {
    let object: Option<Object> = ctx.find_unique(group_model, &teon!({
        "where": record_json_string_to_where_unique(seed_record.record(), group_model)
    }), None, path![]).await.unwrap();
    if object.is_none() {
        seed_record.delete().await.unwrap();
        perform_insert_into_database(dataset, group, record, group_model, ctx.clone()).await;
        return
    }
    let object = object.unwrap();
    let input = insert_or_update_input(dataset, group, record, group_model, ctx.clone()).await;
    object.set_teon(&input).await.unwrap();
    object.save_for_seed_without_required_relation().await.unwrap();
    seed_record.set_record(object_identifier_in_json(&object));
    seed_record.save().await.unwrap();
}

async fn insert_or_update_input(dataset: &DataSet, group: &Group, record: &Record, group_model: &'static Model, ctx: transaction::Ctx) -> Value {
    let mut input = teon!({});
    // nullify exist relations and reset
    for field in group_model.fields().values().filter(|f| f.foreign_key()) {
        input.as_dictionary_mut().unwrap().insert(field.name().to_owned(), Value::Null);
    }
    for (k, v) in record.value.as_dictionary().unwrap() {
        if group_model.field(k).is_some() {
            input.as_dictionary_mut().unwrap().insert(k.to_owned(), v.clone());
        } else if let Some(relation) = group_model.relation(k) {
            if relation.is_required() && relation.has_foreign_key() {
                // setup required relationship
                let that_record_name = v.as_str().unwrap().to_string();
                let that_record_data = DataSetRecord::find_first(teon!({
                    "where": {
                        "group": relation.model_path().join("."),
                        "dataSet": dataset.name.join(".").as_str(),
                        "name": that_record_name,
                    }
                }), ctx.clone()).await.unwrap().unwrap();
                let that_record_identifier_json = that_record_data.record();
                let relation_model = ctx.namespace().model_at_path(&relation.model_path()).unwrap();
                let that_record: Object = ctx.find_unique(relation_model, &teon!({
                    "where": record_json_string_to_where_unique(&that_record_identifier_json, ctx.namespace().model_at_path(&relation.model_path()).unwrap())
                }), None, path![]).await.unwrap().unwrap();
                for (field, reference) in relation.iter() {
                    input.as_dictionary_mut().unwrap().insert(field.to_owned(), that_record.get_value(reference).unwrap());
                }
                // update relation record
                let (_, opposite_relation) = ctx.namespace().opposite_relation(relation);
                let exist_relation_record = DataSetRelation::find_first(teon!({
                    "where": {
                        "OR": [
                            {
                                "dataSet": dataset.name.join(".").as_str(),
                                "groupA": group.name.join(".").as_str(),
                                "relationA": relation.name(),
                                "nameA": record.name.as_str(),
                                "groupB": that_record.model().path().join("."),
                                "nameB": v.as_str().unwrap().to_string(),
                            },
                            {
                                "dataSet": dataset.name.join(".").as_str(),
                                "groupB": group.name.join(".").as_str(),
                                "relationB": relation.name(),
                                "nameB": record.name.as_str(),
                                "groupA": that_record.model().path().join("."),
                                "nameA": v.as_str().unwrap().to_string()
                            }
                        ]
                    }
                }), ctx.clone()).await.unwrap();
                if exist_relation_record.is_none() {
                    let relation_record = DataSetRelation::new(teon!({
                    "dataSet": dataset.name.join(".").as_str(),
                    "groupA": group.name.join(".").as_str(),
                    "relationA": relation.name(),
                    "nameA": record.name.as_str(),
                    "groupB": that_record.model().path().join("."),
                    "relationB": if opposite_relation.is_some() { Value::String(opposite_relation.unwrap().name().to_owned()) } else { Value::Null },
                    "nameB": v.as_str().unwrap().to_string()
                }), ctx.clone()).await.unwrap();
                    relation_record.save().await.unwrap();
                }
            }
        }
    }
    input
}

/// This perform, saves an object into the database. It doesn't setup relationships without
/// required foreign keys.
async fn perform_insert_into_database(dataset: &DataSet, group: &Group, record: &Record, group_model: &'static Model, ctx: transaction::Ctx) {
    let input = insert_or_update_input(dataset, group, record, group_model, ctx.clone()).await;
    let object = ctx.create_object(group_model, &input, None).await.unwrap();
    object.save_for_seed_without_required_relation().await.unwrap();
    let record_object = DataSetRecord::new(teon!({
        "group": group.name.join(".").as_str(),
        "dataSet": dataset.name.join(".").as_str(),
        "name": record.name.as_str(),
        "record": object_identifier_in_json(&object),
    }), ctx.clone()).await.unwrap();
    record_object.save().await.unwrap();
}

fn record_json_string_to_where_unique(json_str: impl AsRef<str>, model: &'static Model) -> Value {
    let json_value: serde_json::Value = serde_json::from_str(json_str.as_ref()).unwrap();
    let json_object = json_value.as_object().unwrap();
    let mut result_teon_value = teon!({});
    for (k, v) in json_object {
        match model.field(k).unwrap().r#type() {
            Type::String => result_teon_value.as_dictionary_mut().unwrap().insert(k.to_owned(), Value::String(v.as_str().unwrap().to_string())),
            Type::ObjectId => result_teon_value.as_dictionary_mut().unwrap().insert(k.to_owned(), Value::ObjectId(ObjectId::parse_str(v.as_str().unwrap()).unwrap())),
            Type::Int => result_teon_value.as_dictionary_mut().unwrap().insert(k.to_owned(), Value::Int(v.as_i64().unwrap() as i32)),
            Type::Int64 => result_teon_value.as_dictionary_mut().unwrap().insert(k.to_owned(), Value::Int64(v.as_i64().unwrap())),
            _ => unreachable!()
        };
    }
    result_teon_value
}

fn object_identifier_in_json(object: &Object) -> String {
    let identifier = object.identifier();
    let mut result = json!({});
    for (k, v) in identifier.as_dictionary().unwrap() {
        match object.model().field(k).unwrap().r#type() {
            Type::ObjectId => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::String(v.as_object_id().unwrap().to_string())),
            Type::String => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::String(v.as_str().unwrap().to_string())),
            Type::Int => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::Number(serde_json::Number::from(v.as_int().unwrap()))),
            Type::Int64 => result.as_object_mut().unwrap().insert(k.to_owned(), serde_json::Value::Number(serde_json::Number::from(v.as_int64().unwrap()))),
            _ => unreachable!(),
        };
    }
    result.to_string()
}

fn ordered_group(groups: &Vec<Group>, ctx: transaction::Ctx) -> Vec<&Group> {
    let mut deps: IndexMap<String, Vec<String>> = IndexMap::new();
    for group in groups {
        let model_name = &group.name.join(".");
        let model = ctx.namespace().model_at_path(&group.name).unwrap();
        let mut model_deps = vec![];
        for relation in model.relations().values() {
            if relation.has_foreign_key() && relation.is_required() {
                model_deps.push(relation.model_path().join("."));
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
                result.push(groups.iter().find(|g| &g.name.join(".") == model_name).unwrap());
                has_some = true;
            }
        }
        if !has_some {
            panic!("Circular required relationship between these models: `{}'.", deps.keys().join(","))
        }
        for group in &result {
            let group_name = group.name.join(".");
            deps.remove(&group_name);
        }
        for (_model_name, model_deps) in deps.iter_mut() {
            for group in &result {
                let group_name = group.name.join(".");
                if let Some(index) = model_deps.iter().position(|x| x == &group_name) {
                    model_deps.remove(index);
                }
            }
        }
    }
    result
}

async fn remove_user_deleted_dataset_records_and_relations(datasets: &Vec<DataSet>, ctx: transaction::Ctx) {
    // remove seed data set records if user removed some seed data set
    let names = Value::Array(datasets.iter().map(|d| Value::String(d.name.join(".").clone())).collect::<Vec<Value>>());
    let records_to_remove = DataSetRecord::find_many(teon!({
        "where": {
            "dataSet": {
                "notIn": &names,
            }
        }
    }), ctx.clone()).await.unwrap();
    for record in records_to_remove {
        record.delete().await.unwrap();
    }
    let relations_to_remove = DataSetRelation::find_many(teon!({
        "where": {
            "dataSet": {
                "notIn": names,
            }
        }
    }), ctx.clone()).await.unwrap();
    for relation in relations_to_remove {
        relation.delete().await.unwrap();
    }
}