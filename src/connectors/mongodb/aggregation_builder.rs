use std::collections::{HashMap, HashSet};
use actix_web::http::header::Accept;
use bson::{Bson, DateTime as BsonDateTime, doc, Document, oid::ObjectId, Regex as BsonRegex};
use chrono::{Date, NaiveDate, Utc, DateTime};
use key_path::{KeyPath, path};
use crate::connectors::shared::has_negative_take::has_negative_take;
use crate::connectors::shared::map_has_i_mode::map_has_i_mode;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::shared::user_json_args::user_json_args;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::{Model};
use crate::core::tson::Value;
use crate::core::error::ActionError;
use crate::core::input::Input;
use crate::core::model::index::ModelIndexType;
use crate::tson;

pub(crate) fn build_unsets_for_match_lookup(model: &Model, _graph: &Graph, r#where: Option<&Value>) -> Result<Vec<Document>, ActionError> {
    if let None = r#where { return Ok(vec![]); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::invalid_query_input("'where' should be an object.")); }
    let r#where = r#where.as_hashmap().unwrap();
    let mut retval: Vec<Document> = vec![];
    for (key, _value) in r#where.iter() {
        let relation = model.relation(key);
        if relation.is_some() {
            retval.push(doc!{"$unset": key})
        }
    }
    Ok(retval)
}

pub(crate) fn build_match_prediction_lookup(model: &Model, graph: &Graph, r#where: Option<&Value>, path: &KeyPath) -> Result<Vec<Document>, ActionError> {
    if let None = r#where { return Ok(vec![]); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::invalid_query_input("'where' should be an object.")); }
    let r#where = r#where.as_hashmap().unwrap();
    let mut include_input = HashMap::new();
    for (key, value) in r#where.iter() {
        let relation = model.relation(key);
        if relation.is_some() {
            let (command, r_where) = Input::key_value(value.as_hashmap().unwrap());
            match command {
                "some" | "is" => {
                    include_input.insert(key.to_string(), tson!({
                        "where": r_where,
                        "take": 1
                    }));
                }
                "none" | "isNot" => {
                    include_input.insert(key.to_string(), tson!({
                        "where": r_where,
                        "take": 1
                    }));
                }
                "all" => {
                    include_input.insert(key.to_string(), tson!({
                        "where": {"NOT": r_where},
                        "take": 1
                    }));
                }
                _ => {}
            }
        }
    }
    Ok(if !include_input.is_empty() {
        build_lookup_inputs(model, graph, QueryPipelineType::Many, false, &Value::HashMap(include_input), path)?
    } else {
        vec![]
    })
}

pub(crate) fn build_where_input(model: &Model, graph: &Graph, r#where: Option<&Value>, path: &KeyPath) -> Result<Document, ActionError> {
    if let None = r#where { return Ok(doc!{}); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::invalid_query_input("'where' should be an object.")); }
    let r#where = r#where.as_hashmap().unwrap();
    let mut doc = doc!{};
    for (key, value) in r#where.iter() {
        if key == "AND" {
            let mut vals: Vec<Document> = vec![];
            for val in value.as_vec().unwrap() {
                vals.push(build_where_input(model, graph, Some(val), &(path + "AND"))?);
            }
            doc.insert("$and", vals);
            continue;
        } else if key == "OR" {
            let mut vals: Vec<Document> = vec![];
            for val in value.as_vec().unwrap() {
                vals.push(build_where_input(model, graph, Some(val), &(path + "OR"))?);
            }
            doc.insert("$or", vals);
            continue;
        } else if key == "NOT" {
            doc.insert("$nor", vec![build_where_input(model, graph, Some(value), &(path + "NOT"))?]);
            continue;
        } else if !model.query_keys().contains(key) {
            return Err(ActionError::unexpected_input_key(key, &(path + key)))
        }
        let field = model.field(key);
        if field.is_some() {

        } else {
            let relation = model.relation(key).unwrap();
            let model_name = relation.model();
            let this_model = graph.model(model_name).unwrap();
            let (command, inner_where) = Input::key_value(value.as_hashmap().unwrap());
            let _inner_where = build_where_input(this_model, graph, Some(inner_where), &(path + key))?;
            match command {
                "none" | "isNot" => {
                    doc.insert(key, doc!{"$size": 0});
                }
                "some" | "is" => {
                    doc.insert(key, doc!{"$size": 1});
                }
                "all" => {
                    doc.insert(key, doc!{"$size": 0});
                }
                _ => {

                }
            }
        }
    }
    Ok(doc)
}

pub(crate) fn build_order_by_input(_model: &Model, _graph: &Graph, order_by: Option<&Value>, reverse: bool, path: &KeyPath) -> Result<Document, ActionError> {
    if order_by.is_none() {
        return Ok(doc!{});
    }
    let order_by = order_by.unwrap();
    let mut retval = doc!{};
    for (index, sort) in order_by.as_vec().unwrap().iter().enumerate() {
        let (key, value) = Input::key_value(sort.as_hashmap().unwrap());
        if value.is_string() {
            let str_val = value.as_str().unwrap();
            if str_val == "asc" {
                retval.insert(key, if reverse { -1 } else { 1 });
            } else if str_val == "desc" {
                retval.insert(key, if reverse { 1 } else { -1 });
            }
        } else {
            return Err(ActionError::unexpected_input_value("'asc' or 'desc'", &(path + index)))
        }
    }
    Ok(retval)
}

fn distinct_key(original: impl AsRef<str>) -> String {
    if original.as_ref() == "_id" {
        "__id".to_string()
    } else {
        original.as_ref().to_string()
    }
}

fn build_select_input(model: &Model, _graph: &Graph, select: &Value, distinct: Option<&Value>) -> Result<Option<Document>, ActionError> {
    let mut true_list: Vec<&str> = vec![];
    let mut false_list: Vec<&str> = vec![];
    let map = select.as_hashmap().unwrap();
    for (key, value) in map {
        let bool_value = value.as_bool().unwrap();
        if bool_value {
            true_list.push(key.as_str());
        } else {
            false_list.push(key.as_str());
        }
    }
    let true_empty = true_list.is_empty();
    let false_empty = false_list.is_empty();
    if true_empty && false_empty {
        if let Some(_distinct) = distinct {
           // go next
        } else {
            return Ok(None);
        }
    }
    if !false_empty || (true_empty && false_empty) {
        // all - false
        let primary_names = model.primary_index().items().iter().map(|i| i.field_name().to_string()).collect::<Vec<String>>();
        let mut keys: HashSet<String> = HashSet::new();
        model.all_keys().iter().for_each(|k| {
            if let Some(field) = model.field(k) {
                let db_name = field.column_name();
                if primary_names.contains(k) {
                    keys.insert(db_name.to_string());
                } else {
                    if !false_list.contains(&&***&k) {
                        keys.insert(db_name.to_string());
                    }
                }
            } else if let Some(property) = model.property(k) {
                if !false_list.contains(&&***&k) {
                    for d in &property.dependencies {
                        let db_name = model.field(d).unwrap().name();
                        keys.insert(db_name.to_string());
                    }
                }
            }
        });
        let mut result = doc!{};
        for key in keys.iter() {
            if distinct.is_some() {
                result.insert(distinct_key(key), doc!{"$first": format!("${key}")});
            } else {
                result.insert(key, 1);
            }
        }
        if result.get("_id").is_none() {
            result.insert("_id", 0);
        }
        return Ok(Some(result));
    } else {
        // true
        let primary_names = model.primary_index().items().iter().map(|i| i.field_name().to_string()).collect::<Vec<String>>();
        let mut keys: HashSet<String> = HashSet::new();
        model.all_keys().iter().for_each(|k| {
            if let Some(field) = model.field(k) {
                let db_name = field.column_name();
                if primary_names.contains(k) {
                    keys.insert(db_name.to_string());
                } else {
                    if true_list.contains(&&***&k) {
                        keys.insert(db_name.to_string());
                    }
                }
            } else if let Some(property) = model.property(k) {
                if true_list.contains(&&***&k) {
                    for d in &property.dependencies {
                        let db_name = model.field(d).unwrap().name();
                        keys.insert(db_name.to_string());
                    }
                }
            }
        });
        let mut result = doc!{};
        for key in keys.iter() {
            if distinct.is_some() {
                result.insert(distinct_key(key), doc!{"$first": format!("${key}")});
            } else {
                result.insert(key, 1);
            }
        }
        if result.get("_id").is_none() {
            result.insert("_id", 0);
        }
        return Ok(Some(result));
    }
}

fn build_lookup_inputs(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    include: &Value,
    path: &KeyPath,
) -> Result<Vec<Document>, ActionError> {
    let include = include.as_hashmap();
    if include.is_none() {
        let model_name = model.name();
        return Err(ActionError::invalid_query_input(format!("'include' on model '{model_name}' is not an object. Please check your input.")));
    }
    let include = include.unwrap();
    let mut retval: Vec<Document> = vec![];
    for (key, value) in include.iter() {
        let relation = model.relation(key);
        if relation.is_none() {
            let model_name = &model.name();
            return Err(ActionError::invalid_query_input(format!("Relation '{key}' on model '{model_name}' is not exist. Please check your input.")));
        }
        let relation = relation.unwrap();
        let relation_name = relation.name();
        let relation_model_name = relation.model();
        let relation_model = graph.model(relation_model_name).unwrap();
        if value.is_bool() || value.is_hashmap() {
            if relation.through().is_none() { // without join table
                let mut let_value = doc!{};
                let mut eq_values: Vec<Document> = vec![];
                for (index, field_name) in relation.fields().iter().enumerate() {
                    let field_name = model.field(field_name).unwrap().column_name();
                    let reference_name = relation.references().get(index).unwrap();
                    let reference_name_column_name = relation_model.field(reference_name).unwrap().column_name();
                    let_value.insert(reference_name, format!("${field_name}"));
                    eq_values.push(doc!{"$eq": [format!("${reference_name_column_name}"), format!("$${reference_name}")]});
                }
                let mut inner_pipeline = if value.is_object() {
                    build_query_pipeline_from_json(relation_model, graph, r#type, mutation_mode, value, &(path + "include" + key))?
                } else {
                    vec![]
                };
                let inner_is_reversed = has_negative_take(value);
                let inner_match = inner_pipeline.iter().find(|v| v.get("$match").is_some());
                let has_inner_match = inner_match.is_some();
                let mut inner_match = if has_inner_match {
                    inner_match.unwrap().clone()
                } else {
                    doc!{"$match": {}}
                };
                let inner_match_inner = inner_match.get_mut("$match").unwrap().as_document_mut().unwrap();
                if inner_match_inner.get("$expr").is_none() {
                    inner_match_inner.insert("$expr", doc!{});
                }
                if inner_match_inner.get("$expr").unwrap().as_document().unwrap().get("$and").is_none() {
                    inner_match_inner.get_mut("$expr").unwrap().as_document_mut().unwrap().insert("$and", vec![] as Vec<Document>);
                }
                inner_match_inner.get_mut("$expr").unwrap().as_document_mut().unwrap().get_mut("$and").unwrap().as_array_mut().unwrap().extend(eq_values.iter().map(|item| Bson::Document(item.clone())));
                if has_inner_match {
                    let index = inner_pipeline.iter().position(|v| v.get("$match").is_some()).unwrap();
                    inner_pipeline.remove(index);
                    inner_pipeline.insert(index, inner_match);
                } else {
                    inner_pipeline.insert(0, inner_match);
                }
                let lookup = doc!{"$lookup": {
                    "from": &relation_model.table_name(),
                    "as": key,
                    "let": let_value,
                    "pipeline": inner_pipeline
                }};
                retval.push(lookup);
                if inner_is_reversed {
                    retval.push(doc!{"$set": {relation_name: {"$reverseArray": format!("${relation_name}")}}});
                }
            } else { // with join table
                let join_model = graph.model(relation.through().as_ref().unwrap()).unwrap();
                let local_relation_on_join_table = join_model.relation(relation.fields().get(0).unwrap()).unwrap();
                let foreign_relation_on_join_table = join_model.relation(relation.references().get(0).unwrap()).unwrap();
                let foreign_model_name = foreign_relation_on_join_table.model();
                let foreign_model = graph.model(foreign_model_name).unwrap();
                let mut outer_let_value = doc!{};
                let mut outer_eq_values: Vec<Document> = vec![];
                let mut inner_let_value = doc!{};
                let mut inner_eq_values: Vec<Document> = vec![];
                for (index, join_table_field_name) in local_relation_on_join_table.fields().iter().enumerate() {
                    let local_unique_field_name = local_relation_on_join_table.references().get(index).unwrap();
                    let local_unique_field_column_name = model.field(local_unique_field_name).unwrap().column_name();
                    outer_let_value.insert(join_table_field_name, format!("${local_unique_field_column_name}"));
                    outer_eq_values.push(doc!{"$eq": [format!("${join_table_field_name}"), format!("$${join_table_field_name}")]});
                }
                for (index, join_table_reference_name) in foreign_relation_on_join_table.fields().iter().enumerate() {
                    let foreign_unique_field_name = foreign_relation_on_join_table.references().get(index).unwrap();
                    let foreign_unique_field_column_name = foreign_model.field(foreign_unique_field_name).unwrap().column_name();
                    inner_let_value.insert(join_table_reference_name, format!("${join_table_reference_name}"));
                    inner_eq_values.push(doc!{"$eq": [format!("${foreign_unique_field_column_name}"), format!("$${join_table_reference_name}")]});
                }
                let mut original_inner_pipeline = if value.is_object() {
                    build_query_pipeline_from_json(foreign_model, graph, QueryPipelineType::Many, false, value, &(path + key))?
                } else {
                    vec![]
                };
                let inner_is_reversed = has_negative_take(value);
                let original_inner_pipeline_immu = original_inner_pipeline.clone();
                let mut inner_match = doc!{
                    "$expr": {
                        "$and": inner_eq_values
                    }
                };
                let original_inner_match = original_inner_pipeline.iter().find(|v| {
                    v.get("$match").is_some()
                });
                if original_inner_match.is_some() {
                    let original_inner_match = original_inner_match.unwrap();
                    let doc = original_inner_match.get_document("$match").unwrap();
                    for (k, v) in doc.iter() {
                        inner_match.insert(k, v);
                    }
                }
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$match").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                    original_inner_pipeline.insert(index.unwrap(), doc!{"$match": inner_match});
                } else {
                    original_inner_pipeline.insert(0, doc!{"$match": inner_match});
                }
                // group addfields unset for distinct
                let original_inner_group = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$group").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$group").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let original_inner_add_fields = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$addFields").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$addFields").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let original_inner_unset = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$unset").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$unset").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let original_inner_sort = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$sort").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$sort").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let original_inner_skip = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$skip").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$skip").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let original_inner_limit = original_inner_pipeline_immu.iter().find(|v| {
                    v.get("$limit").is_some()
                });
                let index = original_inner_pipeline.iter().position(|v| {
                    v.get("$limit").is_some()
                });
                if index.is_some() {
                    original_inner_pipeline.remove(index.unwrap());
                }
                let mut target = doc!{
                    "$lookup": {
                        "from": join_model.table_name(),
                        "as": relation_name,
                        "let": outer_let_value,
                        "pipeline": [{
                            "$match": {
                                "$expr": {
                                    "$and": outer_eq_values
                                }
                            }
                        }, {
                            "$lookup": {
                                "from": foreign_model.table_name(),
                                "as": relation_name,
                                "let": inner_let_value,
                                "pipeline": original_inner_pipeline
                            }
                        }, {
                            "$unwind": {
                                "path": format!("${relation_name}")
                            }
                        }, {
                            "$replaceRoot": {
                                "newRoot": format!("${relation_name}")
                            }
                        }]
                    }
                };
                if original_inner_group.is_some() {
                    let original_inner_group = original_inner_group.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_group.clone()));
                }
                if original_inner_add_fields.is_some() {
                    let original_inner_add_fields = original_inner_add_fields.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_add_fields.clone()));
                }
                if original_inner_unset.is_some() {
                    let original_inner_unset = original_inner_unset.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_unset.clone()));
                }
                if original_inner_sort.is_some() {
                    let original_inner_sort = original_inner_sort.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_sort.clone()));
                }
                if original_inner_skip.is_some() {
                    let original_inner_skip = original_inner_skip.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_skip.clone()));
                }
                if original_inner_limit.is_some() {
                    let original_inner_limit = original_inner_limit.unwrap();
                    target.get_document_mut("$lookup").unwrap().get_array_mut("pipeline").unwrap().push(Bson::Document(original_inner_limit.clone()));
                }
                retval.push(target);
                if inner_is_reversed {
                    retval.push(doc!{"$set": {relation_name: {"$reverseArray": format!("${relation_name}")}}});
                }
            }
        } else {
            let model_name = model.name();
            return Err(ActionError::invalid_query_input(format!("Relation '{key}' on model '{model_name}' has a unrecognized value. It's either a boolean or an object. Please check your input.")));
        }
    }
    Ok(retval)
}

fn insert_group_set_unset_for_aggregate(model: &Model, group: &mut Document, set: &mut Document, unset: &mut Vec<String>, k: &str, g: &str, having_mode: bool) {
    let prefix = if having_mode { "_having" } else { "" };
    let dbk = if k == "_all" { "_all" } else {model.field(k).unwrap().column_name() };
    if g == "count" {
        if k == "_all" {
            group.insert(format!("{prefix}_count__all"), doc!{"$count": {}});
        } else {
            group.insert(format!("{prefix}_count_{dbk}"), doc!{
                "$sum": {
                    "$cond": [{"$ifNull": [format!("${dbk}"), false]}, 1, 0]
                }
            });
        }
    } else {
        group.insert(format!("{prefix}_{g}_{dbk}"), doc!{format!("${g}"): format!("${dbk}")});
        if g == "sum" {
            group.insert(format!("{prefix}_{g}_count_{dbk}"), doc!{format!("$sum"): {
                "$cond": [
                    {"$ifNull": [format!("${dbk}"), false]},
                    1,
                    0
                ]
            }});
        }
    }
    if g == "sum" {
        set.insert(format!("{prefix}_{g}.{k}"), doc!{
            "$cond": {
                "if": {
                    "$eq": [format!("${prefix}_{g}_count_{dbk}"), 0]
                },
                "then": null,
                "else": format!("${prefix}_{g}_{dbk}")
            }
        });
        unset.push(format!("{prefix}_{g}_{dbk}"));
        unset.push(format!("{prefix}_{g}_count_{dbk}"));
    } else {
        set.insert(format!("{prefix}_{g}.{k}"), format!("${prefix}_{g}_{dbk}"));
        unset.push(format!("{prefix}_{g}_{dbk}"));
    }
}

fn build_query_pipeline(
    model: &Model,
    graph: &Graph,
    _type: QueryPipelineType,
    mutation_mode: bool,
    r#where: Option<&Value>,
    order_by: Option<&Value>,
    cursor: Option<&Value>,
    take: Option<i32>,
    skip: Option<i32>,
    page_size: Option<i32>,
    page_number: Option<i32>,
    include: Option<&Value>,
    distinct: Option<&Value>,
    select: Option<&Value>,
    aggregates: Option<&Value>,
    by: Option<&Value>,
    having: Option<&Value>,
    path: &KeyPath,
) -> Result<Vec<Document>, ActionError> {
    // cursor tweaks things so that we validate cursor first
    let cursor_additional_where = if cursor.is_some() {
        let cursor = cursor.unwrap();

        let order_by = order_by.unwrap();
        if !order_by.is_object() {
            return Err(ActionError::invalid_query_input("'orderBy' should be an object."));
        }
        let order_by_map = order_by.as_hashmap().unwrap();
        if order_by_map.len() != 1 {
            return Err(ActionError::invalid_query_input("'orderBy' used with 'cursor' should have a single key which represents a unique constraint."));
        }
        let cursor_map = cursor.as_hashmap().unwrap();
        if cursor_map.len() != 1 {
            return Err(ActionError::invalid_query_input("'cursor' should have a single key which represents a unique constraint."));
        }
        let (order_by_key, order_by_value) = Input::key_value(order_by.as_hashmap().unwrap());
        let (cursor_key, cursor_value) = Input::key_value(cursor.as_hashmap().unwrap());
        if order_by_key != cursor_key {
            return Err(ActionError::invalid_query_input("'cursor' and 'orderBy' should have single same key."));
        }
        if !order_by_value.is_string() {
            return Err(ActionError::invalid_query_input("Field value of 'orderBy' should be one of 'asc' or 'desc'."));
        }
        let order_by_str = order_by_value.as_str().unwrap();
        if order_by_str != "asc" && order_by_str != "desc" {
            return Err(ActionError::invalid_query_input("Field value of 'orderBy' should be one of 'asc' or 'desc'."));
        }

    } else {
        None
    };

    // $build the pipeline
    let mut retval: Vec<Document> = vec![];
    // $lookup for matching
    let lookups_for_matching = build_match_prediction_lookup(model, graph, r#where, path)?;
    if !lookups_for_matching.is_empty() {
        retval.extend(lookups_for_matching);
    }
    // $match
    let r#match = build_where_input(model, graph, r#where, &(path + "where"))?;
    if !r#match.is_empty() {
        if cursor_additional_where.is_some() {
            retval.push(doc!{"$match": {"$and": [r#match, cursor_additional_where.unwrap()]}});
        } else {
            retval.push(doc!{"$match": r#match});
        }
    } else {
        if cursor_additional_where.is_some() {
            retval.push(doc!{"$match": cursor_additional_where.unwrap()});
        }
    }
    // remove lookup for matching here
    let unsets = build_unsets_for_match_lookup(model, graph, r#where)?;
    if !unsets.is_empty() {
        retval.extend(unsets);
    }
    if distinct.is_none() {
        // $sort, if distinct, sort later in distinct
        let reverse = match take {
            Some(take) => take < 0,
            None => false
        };
        let sort = build_order_by_input(model, graph, order_by, reverse, &(path + "orderBy"))?;
        if !sort.is_empty() {
            retval.push(doc!{"$sort": sort});
        }
    }
    // $skip and $limit
    if page_size.is_some() && page_number.is_some() {
        retval.push(doc!{"$skip": ((page_number.unwrap() - 1) * page_size.unwrap()) as i64});
        retval.push(doc!{"$limit": page_size.unwrap() as i64});
    } else {
        if skip.is_some() {
            retval.push(doc!{"$skip": skip.unwrap() as i64});
        }
        if take.is_some() {
            retval.push(doc!{"$limit": take.unwrap().abs() as i64});
        }
    }
    // distinct ($group and $project)
    if let Some(distinct) = distinct {
        // $group
        let mut group_id = doc!{};
        for value in distinct.as_vec().unwrap().iter() {
            let val = value.as_str().unwrap();
            group_id.insert(val, format!("${val}"));
        }
        let empty = tson!({});
        let mut group_data = build_select_input(model, graph, select.unwrap_or(&empty), Some(distinct))?.unwrap();
        group_data.insert("_id", group_id);
        retval.push(doc!{"$group": &group_data});
        if group_data.get("__id").is_some() {
            retval.push(doc!{"$addFields": {"_id": "$__id"}});
            retval.push(doc!{"$unset": "__id"});
        } else {
            retval.push(doc!{"$unset": "_id"});
        }
        // $sort again if distinct
        let reverse = match take {
            Some(take) => take < 0,
            None => false
        };
        let sort = build_order_by_input(model, graph, order_by, reverse, &(path + "orderBy"))?;
        if !sort.is_empty() {
            retval.push(doc!{"$sort": sort});
        }
    } else {
        // $project
        if select.is_some() {
            let select_input = build_select_input(model, graph, select.unwrap(), distinct)?;
            if let Some(select_input) = select_input {
                retval.push(doc!{"$project": select_input})
            }
        }
    }
    // $lookup
    if include.is_some() {
        let mut lookups = build_lookup_inputs(model, graph, QueryPipelineType::Many, mutation_mode, include.unwrap(), path)?;
        if !lookups.is_empty() {
            retval.append(&mut lookups);
        }
    }
    // group by contains it's own aggregates
    let empty_aggregates = tson!({});
    let the_aggregates = if by.is_some() {
        if aggregates.is_none() {
            Some(&empty_aggregates)
        } else {
            aggregates
        }
    } else {
        aggregates
    };
    // $aggregates at last
    if let Some(aggregates) = the_aggregates {
        let mut group = if let Some(by) = by {
            let mut id_for_group_by = doc!{};
            for key in by.as_vec().unwrap() {
                let k = key.as_str().unwrap();
                let dbk = model.field(k).unwrap().column_name();
                id_for_group_by.insert(dbk, doc!{
                    "$cond": [{"$ifNull": [format!("${dbk}"), false]}, format!("${dbk}"), null]
                });
            }
            doc!{"_id": id_for_group_by}
        } else {
            doc!{"_id": Bson::Null}
        };
        let mut set = doc!{};
        let mut unset: Vec<String> = vec![];
        if let Some(by) = by {
            for key in by.as_vec().unwrap() {
                let k = key.as_str().unwrap();
                let dbk = model.field(k).unwrap().column_name();
                set.insert(k, format!("$_id.{dbk}"));
            }
        }
        if let Some(having) = having {
            for (k, o) in having.as_hashmap().unwrap() {
                let _dbk = model.field(k).unwrap().column_name();
                for (g, _matcher) in o.as_hashmap().unwrap() {
                    let g = g.strip_prefix("_").unwrap();
                    insert_group_set_unset_for_aggregate(model, &mut group, &mut set, &mut unset, k, g, true);
                }
            }
        }
        for (g, o) in aggregates.as_hashmap().unwrap() {
            let g = g.strip_prefix("_").unwrap();
            for (k, _t) in o.as_hashmap().unwrap() {
                insert_group_set_unset_for_aggregate(model, &mut group, &mut set, &mut unset, k, g, false);
            }
        }
        retval.push(doc!{"$group": group});
        retval.push(doc!{"$set": set});
        if !unset.is_empty() {
            retval.push(doc!{"$unset": unset});
        }
        // filter if there is a having
        if let Some(having) = having {
            let mut having_match = doc!{};
            let mut having_unset: Vec<String> = Vec::new();
            for (k, o) in having.as_hashmap().unwrap() {
                let dbk = model.field(k).unwrap().column_name();
                for (g, matcher) in o.as_hashmap().unwrap() {
                    let g = g.strip_prefix("_").unwrap();
                    let matcher_bson = parse_bson_where_entry(&FieldType::F64, matcher, graph, &(path + "having" + k + format!("_{g}")))?;
                    having_match.insert(format!("_having_{g}.{dbk}"), matcher_bson);
                    let having_group = format!("_having_{g}");
                    if !having_unset.contains(&having_group) {
                        having_unset.push(having_group);
                    }
                }
            }
            retval.push(doc!{"$match": having_match});
            retval.push(doc!{"$unset": having_unset});
        }
        let mut group_by_sort = doc!{};
        if let Some(by) = by {
            // we need to order these
            for key in by.as_vec().unwrap() {
                let k = key.as_str().unwrap();
                group_by_sort.insert(k, 1);
            }
        }
        if !group_by_sort.is_empty() {
            retval.push(doc!{"$sort": group_by_sort});
        }
    }
    Ok(retval)
}

/// Build MongoDB aggregation pipeline for querying.
/// # Arguments
///
/// * `mutation_mode` - When mutation mode is true, `select` and `include` is ignored.
///
pub(crate) fn build_query_pipeline_from_json(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &Value,
    path: &KeyPath,
) -> Result<Vec<Document>, ActionError> {
    let user_args = user_json_args(model, graph, r#type, mutation_mode, json_value)?;
    let result = build_query_pipeline(
        model,
        graph,
        r#type,
        mutation_mode,
        user_args.r#where,
        user_args.order_by,
        user_args.cursor,
        user_args.take,
        user_args.skip,
        user_args.page_size,
        user_args.page_number,
        user_args.include,
        user_args.distinct,
        user_args.select,
        user_args.aggregates.as_ref(),
        user_args.by,
        user_args.having,
        path);
    result
}
