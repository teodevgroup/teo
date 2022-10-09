use bson::{Bson, doc, Document};
use key_path::KeyPath;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Value};
use crate::tson;

pub(crate) struct Aggregation { }

impl Aggregation {
    pub(crate) fn build(model: &Model, graph: &Graph, r#type: QueryPipelineType, mutation_mode: bool, value: &Value) -> ActionResult<Vec<Document>> {
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
}
