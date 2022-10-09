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
    let cursor_additional_where = None;

    // $build the pipeline
    let mut retval: Vec<Document> = vec![];




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
