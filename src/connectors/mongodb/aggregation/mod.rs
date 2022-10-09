use std::collections::HashMap;
use bson::{Bson, doc, Document};
use key_path::KeyPath;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::core::relation::Relation;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Value};
use crate::tson;

pub(crate) struct Aggregation { }

impl Aggregation {
    pub(crate) fn build(model: &Model, graph: &Graph, r#type: QueryPipelineType, mutation_mode: bool, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval: Vec<Document> = vec![];
        let r#where = value.get("where");
        let skip = value.get("skip");
        let take = value.get("take");
        // if cursor exists, we modify the actual where
        let cursor_where_additions = if let Some(cursor) = value.get("cursor") {
            let cursor = cursor.as_hashmap().unwrap();
            let cursor_value = cursor.values().next().unwrap();
            let order_by = value.get("orderBy").unwrap().as_hashmap().unwrap().values().next().unwrap().as_str().unwrap();
            let mut order_asc = order_by == "asc";
            if let Some(take) = take {
                if take.as_u64().unwrap() < 0 {
                    order_asc = !order_asc;
                }
            }
            let cursor_where_key = if order_asc { "gte" } else { "lte" };
            let cursor_additional_where = Self::build_where(model, graph, &tson!({cursor_key: {cursor_where_key: cursor_value}}));
            Some(cursor_additional_where)
        } else {
            None
        };
        // build `$lookup`s for relation where
        if let Some(r#where) = r#where {
            let lookups_for_relation_where = Self::build_lookups_for_relation_where(model, graph, r#where)?;
            retval.extend(lookups_for_relation_where)
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
            Ok(Bson::Document(map.iter().map(|(k, v)| {
                let k = k.as_str();
                match k {
                    "startsWith" => {
                        let bson_regex = BsonRegex {
                            pattern: "^".to_string() + &*regex::escape(v.as_str().unwrap()),
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        Some(("$regex".to_string(), regex))
                    },
                    "endsWith" => {
                        let bson_regex = BsonRegex {
                            pattern: regex::escape(v.as_str().unwrap()) + "$",
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        Some(("$regex".to_string(), regex))
                    },
                    "contains" => {
                        let bson_regex = BsonRegex {
                            pattern: regex::escape(v.as_str().unwrap()),
                            options: if Input::has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        Some(("$regex".to_string(), regex))
                    },
                    "matches" => {
                        let bson_regex = BsonRegex {
                            pattern: v.as_str().unwrap(),
                            options: if map_has_i_mode(map) { "i".to_string() } else { "".to_string() }
                        };
                        let regex = Bson::RegularExpression(bson_regex);
                        Some(("$regex".to_string(), regex))
                    },
                    "isEmpty" => {
                        Some(("$size", Bson::from(0)))
                    },
                    "mode" => None,
                    _ => Some((Self::build_where_key(k), Bson::from(v)))
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
                    retval.extend(Self::build_lookup_with_join_table(model, graph, relation, value)?)
                } else {
                    retval.extend(Self::build_lookup_without_join_table(model, graph, relation, value)?)
                }
            }
        }
        Ok(retval)
    }

    fn build_lookup_with_join_table(model: &Model, graph: &Graph, relation: &Relation, value: &Value) -> ActionResult<Vec<Document>> {
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
            outer_let_value.insert(join_table_field_name, format!("${local_column_name}"));
            outer_eq_values.push(doc! {"$eq": [format!("${jt_column_name}"), format!("$${jt_column_name}")]});
        }
        for (jt_field, foreign_field) in foreign_relation_on_join_table.iter() {
            let jt_column_name = join_model.field(jt_field).unwrap().column_name();
            let foreign_column_name = model.field(foreign_field).unwrap().column_name();
            inner_let_value.insert(jt_column_name, format!("${jt_column_name}"));
            inner_eq_values.push(doc! {"$eq": [format!("${foreign_column_name}"), format!("$${jt_column_name}")]});
        }
        let mut original_inner_pipeline = if value.is_hashmap() {
            Self::build(opposite_model, graph, QueryPipelineType::Many, false, value)?;
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
            retval.push(doc! {"$set": {relation_name: {"$reverseArray": format!("${}", relation.name())}}});
        }
        Ok(retval)
    }

    fn build_lookup_without_join_table(model: &Model, graph: &Graph, relation: &Relation, value: &Value) -> ActionResult<Vec<Document>> {
        let mut retval = vec![];
        let mut let_value = doc!{};
        let mut eq_values: Vec<Document> = vec![];
        let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
        for (field, reference) in relation.iter() {
            let field_column_name = model.field(field).unwrap().column_name();
            let reference_column_name = opposite_model.field(reference).unwrap().column_name();
            let_value.insert(reference_column_name, format!("${field_column_name}"));
            eq_values.push(doc!{"$eq": [format!("${reference_column_name}"), format!("$${reference_column_name}")]});
        }
        let mut inner_pipeline = if value.is_hashmap() {
            Self::build(opposite_model, graph, QueryPipelineType::Many, false, value)?
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
                "from": &relation_model.table_name(),
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

    fn build_lookups_for_relation_where(model: &Model, graph: &Graph, r#where: &Value) -> ActionResult<Vec<Document>> {
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
}
