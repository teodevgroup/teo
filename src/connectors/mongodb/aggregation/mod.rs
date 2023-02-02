use std::collections::{HashMap, HashSet};
use bson::{Bson, doc, Document, Regex as BsonRegex};
use key_path::KeyPath;
use maplit::hashmap;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::core::relation::Relation;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Value};
use crate::teon;

pub(crate) struct Aggregation { }

impl Aggregation {

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

    pub(crate) fn build_for_aggregate(model: &Model, graph: &Graph, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval = Self::build(model, graph, value)?;
        let by = value.get("by");
        let having = value.get("having");
        let mut aggregates = teon!({});
        for k in ["_sum", "_count", "_avg", "_min", "_max"] {
            if value.as_hashmap().unwrap().contains_key(k) {
                aggregates[k] = value.as_hashmap().unwrap().get(k).unwrap().clone();
            }
        }
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
                    Self::insert_group_set_unset_for_aggregate(model, &mut group, &mut set, &mut unset, k, g, true);
                }
            }
        }
        for (g, o) in aggregates.as_hashmap().unwrap() {
            let g = g.strip_prefix("_").unwrap();
            for (k, _t) in o.as_hashmap().unwrap() {
                Self::insert_group_set_unset_for_aggregate(model, &mut group, &mut set, &mut unset, k, g, false);
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
                    let matcher_bson = Self::build_where_item(model, graph, &FieldType::F64, true, matcher)?;
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
        Ok(retval)
    }

    pub(crate) fn build_for_count(model: &Model, graph: &Graph, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval = Self::build(model, graph, value)?;
        retval.push(doc! {"$count": "count"});
        Ok(retval)
    }

    pub(crate) fn build(model: &Model, graph: &Graph, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval: Vec<Document> = vec![];
        let r#where = value.get("where");
        let order_by = value.get("orderBy");
        let distinct = value.get("distinct");
        let skip = value.get("skip");
        let take = value.get("take");
        let page_size = value.get("pageSize");
        let page_number = value.get("pageNumber");
        let select = value.get("select");
        let include = value.get("include");
        // if cursor exists, we modify the actual where
        let cursor_where_additions = if let Some(cursor) = value.get("cursor") {
            let cursor = cursor.as_hashmap().unwrap();
            let cursor_key = cursor.keys().next().unwrap();
            let cursor_value = cursor.values().next().unwrap();
            let order_by = value.get("orderBy").unwrap().as_vec().unwrap().get(0).unwrap().as_hashmap().unwrap().values().next().unwrap().as_str().unwrap();
            let mut order_asc = order_by == "asc";
            if let Some(take) = take {
                if take.as_i64().unwrap() < 0 {
                    order_asc = !order_asc;
                }
            }
            let cursor_where_key = if order_asc { "gte" } else { "lte" };
            let cursor_additional_where = Self::build_where(model, graph, &teon!({cursor_key: {cursor_where_key: cursor_value}}));
            Some(cursor_additional_where?)
        } else {
            None
        };
        // build `$lookup`s for relation where
        if let Some(r#where) = r#where {
            let lookups_for_relation_where = Self::build_lookups_for_relation_where(model, graph, r#where)?;
            retval.extend(lookups_for_relation_where)
        }
        // $match
        if let Some(r#where) = r#where {
            let r#match = Self::build_where(model, graph, r#where)?;
            if !r#match.is_empty() {
                if let Some(cursor_where_additions) = cursor_where_additions {
                    retval.push(doc!{"$match": {"$and": [r#match, cursor_where_additions]}});
                } else {
                    retval.push(doc!{"$match": r#match});
                }
            } else {
                if let Some(cursor_where_additions) = cursor_where_additions {
                    retval.push(doc!{"$match": cursor_where_additions});
                }
            }
        } else {
            if let Some(cursor_where_additions) = cursor_where_additions {
                retval.push(doc!{"$match": cursor_where_additions});
            }
        }
        // remove lookup for matching here
        if let Some(r#where) = r#where {
            let unsets = Self::build_unsets_for_relation_where(model, r#where)?;
            if !unsets.is_empty() {
                retval.extend(unsets);
            }
        }
        // sort without distinct. If distinct, sort later in distinct
        if distinct.is_none() {
            if let Some(order_by) = order_by {
                let reverse = match take {
                    Some(take) => take.as_i64().unwrap() < 0,
                    None => false
                };
                let sort = Self::build_order_by(model, order_by, reverse)?;
                if !sort.is_empty() {
                    retval.push(doc!{"$sort": sort});
                }
            } else if let Some(take) = take {
                if take.as_i64().unwrap() < 0 {
                    let sort = Self::build_order_by(model, &Self::default_desc_order(model), false)?;
                    retval.push(doc!{"$sort": sort});
                }
            }
        }
        // $skip and $limit
        if page_size.is_some() && page_number.is_some() {
            retval.push(doc!{"$skip": ((page_number.unwrap().as_i64().unwrap() - 1) * page_size.unwrap().as_i64().unwrap()) as i64});
            retval.push(doc!{"$limit": page_size.unwrap().as_i64().unwrap()});
        } else {
            if skip.is_some() {
                retval.push(doc!{"$skip": skip.unwrap().as_i64().unwrap()});
            }
            if take.is_some() {
                retval.push(doc!{"$limit": take.unwrap().as_i64().unwrap().abs()});
            }
        }
        // distinct or select
        // distinct ($group and $project)
        if let Some(distinct) = distinct {
            // $group
            let mut group_id = doc!{};
            for value in distinct.as_vec().unwrap().iter() {
                let val = value.as_str().unwrap();
                group_id.insert(val, format!("${val}"));
            }
            let empty = teon!({});
            let mut group_data = Self::build_select(model, graph, select.unwrap_or(&teon!({})), Some(distinct))?;
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
                Some(take) => take.as_i64().unwrap() < 0,
                None => false
            };
            if let Some(order_by) = order_by {
                let sort = Self::build_order_by(model, order_by, reverse)?;
                if !sort.is_empty() {
                    retval.push(doc!{"$sort": sort});
                }
            }
        } else {
            // $project
            if let Some(select) = select {
                if !select.as_hashmap().unwrap().is_empty() {
                    let select_input = Self::build_select(model, graph, select, distinct)?;
                    if !select_input.is_empty() {
                        retval.push(doc!{"$project": select_input})
                    }
                }
            }
        }
        // $lookup
        if let Some(include) = include {
            let mut lookups = Self::build_lookups(model, graph, include)?;
            if !lookups.is_empty() {
                retval.append(&mut lookups);
            }
        }
        Ok(retval)
    }

    fn build_select(model: &Model, graph: &Graph, select: &Value, distinct: Option<&Value>) -> ActionResult<Document> {
        let map = select.as_hashmap().unwrap();
        let true_keys: Vec<&String> = map.iter().filter(|(k, v)| v.as_bool().unwrap() == true).map(|(k, _)| k).collect();
        let false_keys: Vec<&String> = map.iter().filter(|(k, v)| v.as_bool().unwrap() == false).map(|(k, _)| k).collect();
        let primary_field_names = model.primary_index().keys();
        let mut keys: HashSet<String> = HashSet::new();
        let save_unmentioned_keys = true_keys.is_empty();
        model.all_keys().iter().for_each(|k| {
            let save = primary_field_names.contains(k) || (!false_keys.contains(&k) && (true_keys.contains(&k) || save_unmentioned_keys));
            if save {
                if let Some(field) = model.field(k) {
                    let column_name = field.column_name();
                    keys.insert(column_name.to_string());
                } else if let Some(property) = model.property(k) {
                    for d in &property.dependencies {
                        let column_name = model.field(d).unwrap().name();
                        keys.insert(column_name.to_string());
                    }
                }
            }
        });
        let mut result = doc!{};
        for key in keys.iter() {
            if distinct.is_some() {
                result.insert(Self::distinct_key(key), doc!{"$first": format!("${key}")});
            } else {
                result.insert(key, 1);
            }
        }
        if result.get("_id").is_none() {
            result.insert("_id", 0);
        }
        Ok(result)
    }

    fn build_order_by(model: &Model, order_by: &Value, reverse: bool) -> ActionResult<Document> {
        let mut retval = doc!{};
        for sort in order_by.as_vec().unwrap().iter() {
            let (key, value) = Input::key_value(sort.as_hashmap().unwrap());
            let key = model.field(key).unwrap().column_name();
            if value.is_string() {
                let str_val = value.as_str().unwrap();
                if str_val == "asc" {
                    retval.insert(key, if reverse { -1 } else { 1 });
                } else if str_val == "desc" {
                    retval.insert(key, if reverse { 1 } else { -1 });
                }
            }
        }
        Ok(retval)
    }

    fn build_where(model: &Model, graph: &Graph, value: &Value) -> ActionResult<Document> {
        let value_map = value.as_hashmap().unwrap();
        let mut retval = doc!{};
        for (key, value) in value_map.iter() {
            let key = key.as_str();
            match key {
                "AND" => {
                    let mut vals: Vec<Document> = vec![];
                    for val in value.as_vec().unwrap() {
                        vals.push(Self::build_where(model, graph, val)?);
                    }
                    retval.insert("$and", vals);
                }
                "OR" => {
                    let mut vals: Vec<Document> = vec![];
                    for val in value.as_vec().unwrap() {
                        vals.push(Self::build_where(model, graph, val)?);
                    }
                    retval.insert("$or", vals);
                }
                "NOT" => {
                    retval.insert("$nor", vec![Self::build_where(model, graph, value)?]);
                }
                _ => {
                    if let Some(field) = model.field(key) {
                        let column_name = field.column_name();
                        retval.insert(column_name, Self::build_where_item(model, graph, field.r#type(), field.is_optional(), value)?);
                    } else if let Some(relation) = model.relation(key) {
                        let relation_model = graph.model(relation.model()).unwrap();
                        let (command, inner_where) = Input::key_value(value.as_hashmap().unwrap());
                        let _inner_where = Self::build_where(relation_model, graph, inner_where)?;
                        match command {
                            "none" | "isNot" => {
                                retval.insert(key, doc!{"$size": 0});
                            }
                            "some" | "is" => {
                                retval.insert(key, doc!{"$size": 1});
                            }
                            "all" => {
                                retval.insert(key, doc!{"$size": 0});
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(retval)
    }

    fn build_where_item(model: &Model, graph: &Graph, r#type: &FieldType, optional: bool, value: &Value) -> ActionResult<Bson> {
        if let Some(map) = value.as_hashmap() {
            Ok(Bson::Document(map.iter().filter(|(k, _)| k.as_str() != "mode").map(|(k, v)| {
                let k = k.as_str();
                match k {
                    "startsWith" => {
                        let bson_regex = BsonRegex {
                            pattern: "^".to_string() + &*regex::escape(v.as_str().unwrap()),
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        ("$regex".to_string(), regex)
                    },
                    "endsWith" => {
                        let bson_regex = BsonRegex {
                            pattern: regex::escape(v.as_str().unwrap()) + "$",
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        ("$regex".to_string(), regex)
                    },
                    "contains" => {
                        let bson_regex = BsonRegex {
                            pattern: regex::escape(v.as_str().unwrap()),
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        ("$regex".to_string(), regex)
                    },
                    "matches" => {
                        let bson_regex = BsonRegex {
                            pattern: v.as_str().unwrap().to_string(),
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        ("$regex".to_string(), regex)
                    },
                    "isEmpty" => {
                        ("$size".to_string(), Bson::from(0))
                    },
                    _ => (Self::build_where_key(k).as_str().unwrap().to_string(), Bson::from(v))
                }
            }).collect()))
        } else {
            Ok(Bson::from(value))
        }
    }

    fn build_where_key(key: &str) -> Bson {
        Bson::String(match key {
            "equals" => "$eq",
            "not" => "$ne",
            "gt" => "$gt",
            "gte" => "$gte",
            "lt" => "$lt",
            "lte" => "$lte",
            "in" => "$in",
            "notIn" => "$nin",
            "has" => "$elemMatch",
            "hasEvery" => "$all",
            "hasSome" => "$in",
            "length" => "$size",
            _ => panic!("Unhandled key.")
        }.to_owned())
    }

    fn build_lookups(model: &Model, graph: &Graph, include: &Value) -> ActionResult<Vec<Document>> {
        let include = include.as_hashmap().unwrap();
        let mut retval: Vec<Document> = vec![];
        for (key, value) in include {
            let relation = model.relation(key).unwrap();
            let relation_model = graph.model(relation.model()).unwrap();
            if (value.is_bool() && (value.as_bool().unwrap() == true)) || (value.is_hashmap()) {
                if relation.has_join_table() {
                    retval.extend(Self::build_lookup_with_join_table(model, graph, key, relation, value)?)
                } else {
                    retval.extend(Self::build_lookup_without_join_table(model, graph, key, relation, value)?)
                }
            }
        }
        Ok(retval)
    }

    fn build_lookup_with_join_table(model: &Model, graph: &Graph, key: &str, relation: &Relation, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval = vec![];
        let join_model = graph.model(relation.through().unwrap()).unwrap();
        let local_relation_on_join_table = join_model.relation(relation.local()).unwrap();
        let foreign_relation_on_join_table = join_model.relation(relation.foreign()).unwrap();
        let foreign_model_name = foreign_relation_on_join_table.model();
        let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
        let mut outer_let_value = doc! {};
        let mut outer_eq_values: Vec<Document> = vec![];
        let mut inner_let_value = doc! {};
        let mut inner_eq_values: Vec<Document> = vec![];
        for (jt_field, local_field) in local_relation_on_join_table.iter() {
            let jt_column_name = join_model.field(jt_field).unwrap().column_name();
            let local_column_name = model.field(local_field).unwrap().column_name();
            outer_let_value.insert(jt_column_name, format!("${local_column_name}"));
            outer_eq_values.push(doc! {"$eq": [format!("${jt_column_name}"), format!("$${jt_column_name}")]});
        }
        for (jt_field, foreign_field) in foreign_relation_on_join_table.iter() {
            let jt_column_name = join_model.field(jt_field).unwrap().column_name();
            let foreign_column_name = model.field(foreign_field).unwrap().column_name();
            inner_let_value.insert(jt_column_name, format!("${jt_column_name}"));
            inner_eq_values.push(doc! {"$eq": [format!("${foreign_column_name}"), format!("$${jt_column_name}")]});
        }
        let mut original_inner_pipeline = if value.is_hashmap() {
            Self::build(opposite_model, graph, value)?
        } else {
            vec![]
        };
        let inner_is_reversed = Input::has_negative_take(value);
        let original_inner_pipeline_immu = original_inner_pipeline.clone();
        let mut inner_match = doc! {
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
            original_inner_pipeline.insert(index.unwrap(), doc! {"$match": inner_match});
        } else {
            original_inner_pipeline.insert(0, doc! {"$match": inner_match});
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
        let mut target = doc! {
            "$lookup": {
                "from": join_model.table_name(),
                "as": relation.name(),
                "let": outer_let_value,
                "pipeline": [{
                    "$match": {
                        "$expr": {
                            "$and": outer_eq_values
                        }
                    }
                }, {
                    "$lookup": {
                        "from": opposite_model.table_name(),
                        "as": relation.name(),
                        "let": inner_let_value,
                        "pipeline": original_inner_pipeline
                    }
                }, {
                    "$unwind": {
                        "path": format!("${}", relation.name())
                    }
                }, {
                    "$replaceRoot": {
                        "newRoot": format!("${}", relation.name())
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
            retval.push(doc! {"$set": {relation.name(): {"$reverseArray": format!("${}", relation.name())}}});
        }
        Ok(retval)
    }

    fn build_lookup_without_join_table(model: &Model, graph: &Graph, key: &str, relation: &Relation, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval = vec![];
        let mut let_value = doc!{};
        let mut eq_values: Vec<Document> = vec![];
        let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
        for (field, reference) in relation.iter() {
            let field_name = model.field(field).unwrap().name();
            let field_column_name = model.field(field).unwrap().column_name();
            let reference_name = opposite_model.field(reference).unwrap().name();
            let reference_column_name = opposite_model.field(reference).unwrap().column_name();
            let_value.insert(reference_name, format!("${field_column_name}"));
            eq_values.push(doc!{"$eq": [format!("${reference_column_name}"), format!("$${reference_name}")]});
        }
        let mut inner_pipeline = if value.is_hashmap() {
            Self::build(opposite_model, graph, value)?
        } else {
            vec![]
        };
        let inner_is_reversed = Input::has_negative_take(value);
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
        let lookup = doc!{
            "$lookup": {
                "from": opposite_model.table_name(),
                "as": key,
                "let": let_value,
                "pipeline": inner_pipeline
            }
        };
        retval.push(lookup);
        if inner_is_reversed {
            retval.push(doc!{"$set": {relation.name(): {"$reverseArray": format!("${}", relation.name())}}});
        }
        Ok(retval)
    }

    fn build_unsets_for_relation_where(model: &Model, r#where: &Value) -> ActionResult<Vec<Document>> {
        let r#where = r#where.as_hashmap().unwrap();
        let mut retval: Vec<Document> = vec![];
        for (key, _) in r#where.iter() {
            if let Some(_) = model.relation(key) {
                retval.push(doc!{"$unset": key})
            }
        }
        Ok(retval)
    }

    fn build_lookups_for_relation_where(model: &Model, graph: &Graph, r#where: &Value) -> ActionResult<Vec<Document>> {
        let r#where = r#where.as_hashmap().unwrap();
        let mut include_input = HashMap::new();
        for (key, value) in r#where.iter() {
            let relation = model.relation(key);
            if relation.is_some() {
                let (command, r_where) = Input::key_value(value.as_hashmap().unwrap());
                match command {
                    "some" | "is" => {
                        include_input.insert(key.to_string(), teon!({
                        "where": r_where,
                        "take": 1
                    }));
                    }
                    "none" | "isNot" => {
                        include_input.insert(key.to_string(), teon!({
                        "where": r_where,
                        "take": 1
                    }));
                    }
                    "all" => {
                        include_input.insert(key.to_string(), teon!({
                        "where": {"NOT": r_where},
                        "take": 1
                    }));
                    }
                    _ => {}
                }
            }
        }
        Ok(if !include_input.is_empty() {
            Self::build_lookups(model, graph, &Value::HashMap(include_input))?
        } else {
            vec![]
        })
    }

    fn distinct_key(original: impl AsRef<str>) -> String {
        if original.as_ref() == "_id" {
            "__id".to_string()
        } else {
            original.as_ref().to_string()
        }
    }

    fn default_desc_order(model: &Model) -> Value {
        let mut vec: Vec<Value> = vec![];
        for item in model.primary_index().items() {
            vec.push(Value::HashMap(hashmap!{item.field_name().to_string() => Value::String("desc".to_string())}));
        }
        Value::Vec(vec)
    }
}
