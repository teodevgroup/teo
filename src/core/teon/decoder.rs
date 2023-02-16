use std::collections::{HashSet, HashMap, BTreeMap};
use std::ops::BitOr;
use std::str::FromStr;
#[cfg(feature = "data-source-mongodb")]
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, Utc};
use key_path::{KeyPath, path};
use maplit::{hashmap, hashset};
use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use serde_json::{Value as JsonValue, Map as JsonMap};
use crate::core::action::{Action, CONNECT, CONNECT_OR_CREATE, CREATE, CREATE_MANY_HANDLER, DELETE, DISCONNECT, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, MANY, NESTED, SET, SINGLE, UPDATE, UPSERT};
use crate::core::error::Error;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::model::Model;
use crate::core::result::Result;
use crate::core::graph::Graph;
use crate::core::relation::Relation;
use crate::core::teon::Value;

pub(crate) struct Decoder { }

impl Decoder {

    pub(crate) fn decode_object(model: &Model, graph: &Graph, json_value: &JsonValue) -> Result<Value> {
        Self::decode_object_at_path(model, graph, json_value, path![])
    }

    pub(crate) fn decode_action_arg(model: &Model, graph: &Graph, action: Action, json_value: &JsonValue) -> Result<Value> {
        Self::decode_action_arg_at_path(model, graph, action, json_value, path![])
    }

    fn decode_object_at_path<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &model.all_keys().iter().map(|k| k.as_str()).collect(), path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let path = path + k;
            if let Some(field) = model.field(k) {
                Ok((k.to_owned(), Self::decode_value_for_field_type(graph, field.field_type(), field.is_optional(), v, path)?))
            } else if let Some(relation) = model.relation(k) {
                if relation.is_vec() {
                    Ok((k.to_owned(), Self::decode_nested_many_create_arg(graph, relation, v, path)?))
                } else {
                    Ok((k.to_owned(), Self::decode_nested_one_create_arg(graph, relation, v, path)?))
                }
            } else if let Some(property) = model.property(k) {
                Ok((k.to_owned(), Self::decode_value_for_field_type(graph, property.field_type(), property.is_optional(), v, path)?))
            } else {
                unreachable!()
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_action_arg_at_path<'a>(model: &Model, graph: &Graph, action: Action, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_root_type("object"));
        };
        Self::check_json_keys(json_map, action.handler_allowed_input_json_keys(), path)?;
        let mut retval: HashMap<String, Value> = hashmap!{};
        for (key, value) in json_map {
            let key = key.as_str();
            let path = path + key;
            match key {
                "where" => if action.handler_requires_where() {
                    retval.insert(key.to_owned(), Self::decode_where(model, graph, value, path)?);
                } else if action.handler_requires_where_unique() {
                    retval.insert(key.to_owned(), Self::decode_where_unique(model, graph, value, path)?);
                },
                "orderBy" => { retval.insert(key.to_owned(), Self::decode_order_by(model, value, path)?); }
                "cursor" => { retval.insert(key.to_owned(), Self::decode_where_unique(model, graph, value, path)?); }
                "distinct" => { retval.insert(key.to_owned(), Self::decode_distinct(model, value, path)?); }
                "skip" | "pageSize" | "pageNumber" => { retval.insert(key.to_owned(), Self::decode_usize(value, path)?); }
                "take" => { retval.insert(key.to_owned(), Self::decode_i64(value, path)?); }
                "select" => { retval.insert(key.to_owned(), Self::decode_select(model, value, path)?); }
                "include" => { retval.insert(key.to_owned(), Self::decode_include(model, graph, value, path)?); }
                "_avg" | "_sum" | "_min" | "_max" | "_count" => { retval.insert(key.to_owned(), Self::decode_aggregate(model, key, value, path)?); }
                "by" => { retval.insert(key.to_owned(), Self::decode_by(model, value, path)?); }
                "having" => { retval.insert(key.to_owned(), Self::decode_having(model, graph, value, path)?); }
                "create" => { retval.insert(key.to_owned(), if action.to_u32() == CREATE_MANY_HANDLER { Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_create(model, graph, v, p))? } else { Self::decode_create(model, graph, value, path)? } ); }
                "update" => { retval.insert(key.to_owned(), Self::decode_update(model, graph, value, path)?); }
                "credentials" => { retval.insert(key.to_owned(), Self::decode_credentials(model, graph, value, path)?); }
                _ => unreachable!()
            }
        }
        if retval.contains_key("skip") || retval.contains_key("take") {
            for k in ["pageSize", "pageNumber"] {
                if retval.contains_key(k) {
                    return Err(Error::unexpected_input_key(k, path))
                }
            }
        }
        Ok(Value::HashMap(retval))
    }

    fn check_json_keys<'a>(map: &JsonMap<String, JsonValue>, allowed: &HashSet<&str>, path: &KeyPath<'a>) -> Result<()> {
        if let Some(unallowed) = map.keys().find(|k| !allowed.contains(k.as_str())) {
            return Err(Error::unexpected_input_key(unallowed, path + unallowed));
        }
        Ok(())
    }

    pub(crate) fn check_length_1<'a, 'b>(json_value: &'a JsonValue, path: impl AsRef<KeyPath<'b>>) -> Result<(&'a str, &'a JsonValue)> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            if json_map.len() != 1 {
                Err(Error::unexpected_object_length(1, path))
            } else {
                Ok((json_map.keys().next().unwrap(), json_map.values().next().unwrap()))
            }
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_credentials<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        if let Some(map) = json_value.as_object() {
            let by_set = model.auth_by_keys().iter().map(|k| k.as_str()).collect::<HashSet<&str>>();
            let identity_set = model.auth_identity_keys().iter().map(|k| k.as_str()).collect::<HashSet<&str>>();
            let allowed = by_set.bitor(&identity_set);
            Self::check_json_keys(map, &allowed, path.as_ref())?;
            Ok(Self::decode_create(model, graph, json_value, path.as_ref())?)
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_create<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &model.input_keys().iter().map(|k| k.as_str()).collect(), path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let path = path + k;
            if let Some(field) = model.field(k) {
                Ok((k.to_owned(), Self::decode_value_for_field_type(graph, field.field_type(), field.is_optional(), v, path)?))
            } else if let Some(relation) = model.relation(k) {
                if relation.is_vec() {
                    Ok((k.to_owned(), Self::decode_nested_many_create_arg(graph, relation, v, path)?))
                } else {
                    Ok((k.to_owned(), Self::decode_nested_one_create_arg(graph, relation, v, path)?))
                }
            } else if let Some(property) = model.property(k) {
                Ok((k.to_owned(), Self::decode_value_for_field_type(graph, property.field_type(), property.is_optional(), v, path)?))
            } else {
                panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_many_create_arg<'a>(graph: &Graph, relation: &Relation, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &NESTED_CREATE_MANY_ARG_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, value)| {
            let k = k.as_str();
            let path = path + k;
            let (model, relation) = graph.opposite_relation(relation);
            match k {
                "create" | "createMany" => {
                    if model.has_action(Action::from_u32(NESTED | CREATE | MANY)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_create_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "connect" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where_unique(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "connectOrCreate" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT_OR_CREATE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_connect_or_create_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                _ => unreachable!()
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_many_update_arg<'a>(graph: &Graph, relation: &Relation, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &NESTED_UPDATE_MANY_ARG_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, value)| {
            let k = k.as_str();
            let path = path + k;
            let (model, relation) = graph.opposite_relation(relation);
            match k {
                "create" | "createMany" => {
                    if model.has_action(Action::from_u32(NESTED | CREATE | MANY)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_create_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                },
                "connect" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where_unique(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                },
                "set" => {
                    if model.has_action(Action::from_u32(NESTED | SET | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where_unique(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "disconnect" => {
                    if model.has_action(Action::from_u32(NESTED | DISCONNECT | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where_unique(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "delete" => {
                    if model.has_action(Action::from_u32(NESTED | DELETE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where_unique(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "connectOrCreate" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT_OR_CREATE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_connect_or_create_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "update" => {
                    if model.has_action(Action::from_u32(NESTED | UPDATE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_update_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "updateMany" => {
                    if model.has_action(Action::from_u32(NESTED | UPDATE | MANY)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_update_many_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "deleteMany" => {
                    if model.has_action(Action::from_u32(NESTED | DELETE | MANY)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_where(model, graph, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                "upsert" => {
                    if model.has_action(Action::from_u32(NESTED | UPSERT | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_enumerate(value, path, |v, p: &KeyPath| Self::decode_nested_upsert_input(model, graph, relation, v, p))?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                }
                _ => unreachable!()
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_enumerate<'a, F: Fn(&JsonValue, &KeyPath) -> Result<Value>>(json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>, f: F) -> Result<Value> {
        let path = path.as_ref();
        if let Some(_) = json_value.as_object() {
            f(json_value, path)
        } else if let Some(json_array) = json_value.as_array() {
            Ok(Value::Vec(json_array.iter().enumerate().map(|(i, v)| {
                f(v, &(path + i))
            }).collect::<Result<Vec<Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object or array", path))
        }
    }

    fn decode_nested_one_create_arg<'a>(graph: &Graph, relation: &Relation, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &NESTED_CREATE_ONE_ARG_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            let (model, relation) = graph.opposite_relation(relation);
            match k {
                "create" => {
                    if model.has_action(Action::from_u32(NESTED | CREATE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_nested_create_input(model, graph, relation, v, path)?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                },
                "connect" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                },
                "connectOrCreate" => {
                    if model.has_action(Action::from_u32(NESTED | CONNECT_OR_CREATE | SINGLE)) {
                        Ok((k.to_owned(), Self::decode_nested_connect_or_create_input(model, graph, relation, v, path)?))
                    } else {
                        Err(Error::unexpected_input_key(k, &path))?
                    }
                },
                _ => unreachable!()
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_one_update_arg<'a>(graph: &Graph, relation: &Relation, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &NESTED_UPDATE_ONE_ARG_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            let (model, relation) = graph.opposite_relation(relation);
            match k {
                "create" => if model.has_action(Action::from_u32(CREATE | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_nested_create_input(model, graph, relation, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "connect" => if model.has_action(Action::from_u32(CONNECT | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "set" => if model.has_action(Action::from_u32(SET | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "connectOrCreate" => if model.has_action(Action::from_u32(CONNECT_OR_CREATE | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_nested_connect_or_create_input(model, graph, relation, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "disconnect" => if model.has_action(Action::from_u32(DISCONNECT | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_bool(v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "delete" => if model.has_action(Action::from_u32(DELETE | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_bool(v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                "update" => if model.has_action(Action::from_u32(UPDATE | NESTED | SINGLE)) {
                    Ok((k.to_owned(), Self::decode_nested_inner_update_input(model, graph, relation, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, &path))?
                },
                _ => unreachable!()
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_upsert_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        Self::check_json_keys(json_map, &NESTED_UPSERT_INPUT_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            match k {
                "create" => Ok((k.to_owned(), Self::decode_nested_create_input(model, graph, relation, v, path)?)),
                "update" => Ok((k.to_owned(), Self::decode_nested_inner_update_input(model, graph, relation, v, path)?)),
                "where" => Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?)),
                _ => panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_connect_or_create_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        Self::check_json_keys(json_map, &NESTED_CONNECT_OR_CREATE_INPUT_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            match k {
                "create" => Ok((k.to_owned(), Self::decode_nested_create_input(model, graph, relation, v, path)?)),
                "where" => Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?)),
                _ => panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_update_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        Self::check_json_keys(json_map, &NESTED_UPDATE_INPUT_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            match k {
                "update" => Ok((k.to_owned(), Self::decode_nested_inner_update_input(model, graph, relation, v, path)?)),
                "where" => Ok((k.to_owned(), Self::decode_where_unique(model, graph, v, path)?)),
                _ => panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }
    
    fn decode_nested_update_many_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        Self::check_json_keys(json_map, &NESTED_UPDATE_INPUT_KEYS, path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let k = k.as_str();
            let path = path + k;
            match k {
                "update" => Ok((k.to_owned(), Self::decode_nested_inner_update_input(model, graph, relation, v, path)?)),
                "where" => Ok((k.to_owned(), Self::decode_where(model, graph, v, path)?)),
                _ => panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_nested_inner_update_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        let without: Vec<&str> = if let Some(relation) = relation {
            let mut without = vec![relation.name()];
            without.extend(relation.fields().iter().map(|k| k.as_str()));
            without
        } else {
            vec![]
        };
        let allowed = model.input_keys().iter().filter(|k| {
            !without.contains(&k.as_str())
        }).map(|k| k.as_str()).collect();
        Self::check_json_keys(json_map, &allowed, path)?;
        Self::decode_update(model, graph, &json_value, path)
    }

    fn decode_nested_create_input<'a>(model: &Model, graph: &Graph, relation: Option<&Relation>, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = match json_value.as_object() {
            Some(json_map) => json_map,
            None => return Err(Error::unexpected_input_type("object", path))
        };
        let without: Vec<&str> = if let Some(relation) = relation {
            let mut without = vec![relation.name()];
            if relation.has_foreign_key() {
                without.extend(relation.fields().iter().map(|k| k.as_str()));
            }
            without
        } else {
            vec![]
        };
        let allowed = model.input_keys().iter().filter(|k| {
            !without.contains(&k.as_str())
        }).map(|k| k.as_str()).collect();
        Self::check_json_keys(json_map, &allowed, path)?;
        Self::decode_create(model, graph, &json_value, path)
    }

    fn decode_update<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        Self::check_json_keys(json_map, &model.input_keys().iter().map(|k| k.as_str()).collect(), path)?;
        Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
            let path = path + k;
            if let Some(field) = model.field(k) {
                Ok((k.to_owned(), Self::decode_value_or_updator_for_field_type(graph, field.field_type(), field.is_optional(), v, path, false)?))
            } else if let Some(relation) = model.relation(k) {
                if relation.is_vec() {
                    Ok((k.to_owned(), Self::decode_nested_many_update_arg(graph, relation, v, path)?))
                } else {
                    Ok((k.to_owned(), Self::decode_nested_one_update_arg(graph, relation, v, path)?))
                }
            } else if let Some(property) = model.property(k) {
                Ok((k.to_owned(), Self::decode_value_or_updator_for_field_type(graph, property.field_type(), property.is_optional(), v, path, true)?))
            } else {
                panic!("Unhandled key.")
            }
        }).collect::<Result<HashMap<String, Value>>>()?))
    }

    fn decode_having<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Self::check_json_keys(json_map, &model.scalar_keys().iter().map(|k| k.as_str()).collect(), path)?;
            Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
                let path = path + k;
                let field = model.field(k).unwrap();
                Ok((k.clone(), Self::decode_where_with_aggregates_for_field(graph, field.field_type(), field.is_optional(), v, &path)?))
            }).collect::<Result<HashMap<String, Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_by<'a>(model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_array) = json_value.as_array() {
            Ok(Value::Vec(json_array.iter().enumerate().map(|(i, v)| {
                let path = path + i;
                match v.as_str() {
                    Some(s) => if model.scalar_keys().contains(&s.to_string()) {
                        Ok(Value::String(s.to_owned()))
                    } else {
                        Err(Error::unexpected_input_value("scalar field name", path))
                    }
                    None => Err(Error::unexpected_input_type("string", path))
                }
            }).collect::<Result<Vec<Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("array", path))
        }
    }

    fn decode_aggregate<'a>(model: &Model, key: &str, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Self::check_json_keys(json_map, &model.allowed_keys_for_aggregate(key), path)?;
            Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
                Ok((k.to_owned(), Self::decode_bool(v, path + k)?))
            }).collect::<Result<HashMap<String, Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_include<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
                let path = path + k;
                if model.relation_output_keys().contains(k) {
                    Ok((k.to_owned(), Self::decode_include_item(model, graph, k, v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, path))
                }
            }).collect::<Result<HashMap<String, Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_include_item<'a>(model: &Model, graph: &Graph, name: &str, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(b) = json_value.as_bool() {
            Ok(Value::Bool(b))
        } else if let Some(_json_map) = json_value.as_object() {
            let relation = model.relation(name).unwrap();
            let model = graph.model(relation.model()).unwrap();
            if relation.is_vec() {
                Ok(Self::decode_action_arg_at_path(model, graph, Action::from_u32(FIND_MANY_HANDLER), json_value, path)?)
            } else {
                Ok(Self::decode_action_arg_at_path(model, graph, Action::from_u32(FIND_UNIQUE_HANDLER), json_value, path)?)
            }
        } else {
            Err(Error::unexpected_input_type("bool or object", path))
        }
    }

    fn decode_select<'a>(model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
                let path = path + k;
                if model.local_output_keys().contains(k) {
                    Ok((k.to_owned(), Self::decode_bool(v, path)?))
                } else {
                    Err(Error::unexpected_input_key(k, path))
                }
            }).collect::<Result<HashMap<String, Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_usize<'a>(json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(u) = json_value.as_u64() {
            Ok(Value::I64(u as i64))
        } else {
            Err(Error::unexpected_input_type("positive integer number", path))
        }
    }

    fn decode_i64<'a>(json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(u) = json_value.as_i64() {
            Ok(Value::I64(u))
        } else {
            Err(Error::unexpected_input_type("integer number", path))
        }
    }

    fn decode_bool<'a>(json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(b) = json_value.as_bool() {
            Ok(Value::Bool(b))
        } else {
            Err(Error::unexpected_input_type("bool", path))
        }
    }

    fn decode_distinct<'a>(model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(_) = json_value.as_str() {
            Ok(Self::decode_distinct_item(model, json_value, path)?)
        } else if let Some(json_array) = json_value.as_array() {
            Ok(Value::Vec(json_array.iter().enumerate().map(|(i, v)| {
                Self::decode_distinct_item(model, v, path + i)
            }).collect::<Result<Vec<Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("string or array", path))
        }
    }

    fn decode_distinct_item<'a>(model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        if let Some(s) = json_value.as_str() {
            if model.scalar_keys().contains(&s.to_string()) {
                Ok(Value::String(s.to_owned()))
            } else {
                Err(Error::unexpected_input_value("scalar fields enum", path))
            }
        } else {
            Err(Error::unexpected_input_type("string", path))
        }
    }

    fn decode_order_by<'a>(model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(_) = json_value.as_object() {
            Ok(Value::Vec(vec![Self::decode_order_by_item(model, json_value, path)?]))
        } else if let Some(json_array) = json_value.as_array() {
            Ok(Value::Vec(json_array.iter().enumerate().map(|(i, v)| {
                Self::decode_order_by_item(model, v, path + i)
            }).collect::<Result<Vec<Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("object or array", path))
        }
    }

    fn decode_order_by_item<'a>(_model: &Model, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(_json_map) = json_value.as_object() {
            let (key, value) = Self::check_length_1(json_value, path)?;
            match value.as_str() {
                Some(s) => match s {
                    "asc" | "desc" => Ok(Value::HashMap(hashmap!{key.to_owned() => Value::String(s.to_owned())})),
                    _ => Err(Error::unexpected_input_type("string", path))
                },
                None => Err(Error::unexpected_input_type("string", path))
            }
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_where<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        let mut retval: HashMap<String, Value> = hashmap!{};
        for (key, value) in json_map {
            let key = key.as_str();
            match key {
                "AND" | "OR" => {
                    let path = &(path + key);
                    match value {
                        JsonValue::Object(_) => {
                            retval.insert(key.to_owned(), Self::decode_where(model, graph, value, path)?);
                        }
                        JsonValue::Array(inner_array) => {
                            retval.insert(key.to_owned(), Value::Vec(inner_array.iter().enumerate().map(|(i, v)| {
                                Self::decode_where(model, graph, v, path + i)
                            }).collect::<Result<Vec<Value>>>()?));
                        }
                        _ => {
                            return Err(Error::unexpected_input_type("object or array", path));
                        }
                    }
                }
                "NOT" => {
                    let path = path + key;
                    match value {
                        JsonValue::Object(_) => {
                            retval.insert(key.to_owned(), Self::decode_where(model, graph, value, path)?);
                        }
                        _ => {
                            return Err(Error::unexpected_input_type("object", path));
                        }
                    }
                }
                _ => {
                    let path = path + key;
                    if !model.query_keys().contains(&key.to_string()) {
                        return Err(Error::unexpected_input_key(key, path));
                    }
                    if let Some(field) = model.field(key) {
                        let optional = field.optionality.is_optional();
                        retval.insert(key.to_owned(), Self::decode_where_for_field(graph, field.field_type(), optional, value, path)?);
                    } else if let Some(relation) = model.relation(key) {
                        retval.insert(key.to_owned(), Self::decode_where_for_relation(graph, relation, value, path)?);
                    }
                }
            }
        }
        Ok(Value::HashMap(retval))
    }

    fn decode_where_unique<'a>(model: &Model, graph: &Graph, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        let json_map = if let Some(json_map) = json_value.as_object() {
            json_map
        } else {
            return Err(Error::unexpected_input_type("object", path));
        };
        if json_map.len() == 0 {
            return Err(Error::unexpected_input_value_with_reason("Unique where can't be empty.", path));
        }
        for index in model.indices() {
            if index.keys() == &json_map.keys().into_iter().map(|k| k.to_owned()).collect::<Vec<String>>() {
                let mut retval: HashMap<String, Value> = HashMap::new();
                for (key, value) in json_map {
                    let field = model.field(key).unwrap();
                    let path = path + key;
                    retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, field.field_type(), field.is_optional(), value, path)?);
                    return Ok(Value::HashMap(retval));
                }
            }
        }
        Err(Error::unexpected_input_key(json_map.keys().next().unwrap(), path))
    }

    fn decode_where_for_field_internal<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>, aggregate: bool) -> Result<Value> {
        let path = path.as_ref();
        if json_value.is_object() {
            let json_map = json_value.as_object().unwrap();
            Self::check_json_keys(json_map, if aggregate { r#type.filters_with_aggregates() } else { r#type.filters() }, path)?;
            let mut retval: HashMap<String, Value> = hashmap!{};
            for (key, value) in json_map {
                let key = key.as_str();
                let path = path + key;
                match key {
                    "equals" => {
                        retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, r#type, optional, value, path)?);
                    }
                    "not" => {
                        retval.insert(key.to_owned(), Self::decode_where_for_field(graph, r#type, optional, value, path)?);
                    }
                    "gt" | "gte" | "lt" | "lte" | "contains" | "startsWith" | "endsWith" | "matches" => {
                        retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, r#type, false, value, path)?);
                    }
                    "in" | "notIn" => {
                        retval.insert(key.to_owned(), Self::decode_value_array_for_field_type(graph, r#type, false, value, path)?);
                    }
                    "mode" => match value.as_str() {
                        Some(s) => if s == "caseInsensitive" {
                            retval.insert(key.to_owned(), Value::String("caseInsensitive".to_owned()));
                        } else {
                            return Err(Error::unexpected_input_type("'caseInsensitive'", path));
                        },
                        None => return Err(Error::unexpected_input_type("string", path)),
                    }
                    "has" => {
                        let element_field = r#type.element_field().unwrap();
                        retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, element_field.field_type(), element_field.is_optional(), value, path)?);
                    }
                    "hasEvery" | "hasSome" => {
                        let element_field = r#type.element_field().unwrap();
                        retval.insert(key.to_owned(), Self::decode_value_array_for_field_type(graph, element_field.field_type(), element_field.is_optional(), value, path)?);
                    }
                    "isEmpty" => {
                        retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, &FieldType::Bool, false, value, path)?);
                    }
                    "length" => {
                        retval.insert(key.to_owned(), Self::decode_value_for_field_type(graph, &FieldType::I64, false, value, path)?);
                    }
                    "_avg" | "_sum" => {
                        retval.insert(key.to_owned(), Self::decode_where_for_field(graph, &FieldType::I64, true, value, path)?);
                    }
                    "_count" => {
                        retval.insert(key.to_owned(), Self::decode_where_for_field(graph, &FieldType::I64, false, value, path)?);
                    }
                    "_min" | "_max" => {
                        retval.insert(key.to_owned(), Self::decode_where_for_field(graph, r#type, optional, value, path)?);
                    }
                    _ => return Err(Error::unexpected_input_key(key, path))
                }
            }
            Ok(Value::HashMap(retval))
        } else {
            Ok(Value::HashMap(hashmap!{"equals".to_owned() => Self::decode_value_for_field_type(graph, r#type, optional, json_value, path)?}))
        }
    }

    fn decode_where_with_aggregates_for_field<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        Self::decode_where_for_field_internal(graph, r#type, optional, json_value, path, true)
    }

    fn decode_where_for_field<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        Self::decode_where_for_field_internal(graph, r#type, optional, json_value, path, false)
    }

    fn decode_where_for_relation<'a>(graph: &Graph, relation: &Relation, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Self::check_json_keys(json_map, relation.filters(), path)?;
            let mut retval: HashMap<String, Value> = hashmap!{};
            for (key, value) in json_map {
                let key = key.as_str();
                let path = path + key;
                let model = graph.model(relation.model()).unwrap();
                retval.insert(key.to_owned(), Self::decode_where(model, graph, value, path)?);
            }
            Ok(Value::HashMap(retval))
        } else {
            Err(Error::unexpected_input_type("object", path))
        }
    }

    fn decode_value_array_for_field_type<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        let path = path.as_ref();
        if let Some(array) = json_value.as_array() {
            Ok(Value::Vec(array.iter().enumerate().map(|(i, v)| {
                Self::decode_value_for_field_type(graph, r#type, optional, v, path + i)
            }).collect::<Result<Vec<Value>>>()?))
        } else {
            Err(Error::unexpected_input_type("array", path))
        }
    }

    fn decode_value_or_updator_for_field_type<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>, set_only: bool) -> Result<Value> {
        let path = path.as_ref();
        if let Some(json_map) = json_value.as_object() {
            Self::check_length_1(json_value, path)?;
            Self::check_json_keys(json_map, if set_only { r#type.default_updators() } else { r#type.updators() }, path)?;
            Ok(Value::HashMap(json_map.iter().map(|(k, v)| {
                let k = k.as_str();
                let path = path + k;
                Ok((k.to_owned(), match k {
                    "set" => Self::decode_value_for_field_type(graph, r#type, optional, v, path)?,
                    "increment" | "decrement" | "multiply" | "divide" => Self::decode_value_for_field_type(graph, r#type, false, v, path)?,
                    "push" => {
                        let element_field = r#type.element_field().unwrap();
                        Self::decode_value_for_field_type(graph, element_field.field_type(), element_field.is_optional(), v, path)?
                    }
                    _ => panic!("Unknown updator name.")
                }))
            }).collect::<Result<HashMap<String, Value>>>()?))
        } else {
            Self::decode_value_for_field_type(graph, r#type, optional, json_value, path)
        }
    }

    pub(crate) fn decode_value_for_field_type<'a>(graph: &Graph, r#type: &FieldType, optional: bool, json_value: &JsonValue, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        if optional && json_value.is_null() {
            return Ok(Value::Null);
        }
        let path = path.as_ref();
        match r#type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => match json_value.as_str() {
                Some(str) => match ObjectId::from_str(str) {
                    Ok(oid) => Ok(Value::ObjectId(oid)),
                    Err(_) => Err(Error::unexpected_input_value("object id", path))
                },
                None => Err(Error::unexpected_input_type("object id string", path))
            }
            FieldType::Bool => match json_value.as_bool() {
                Some(b) => Ok(Value::Bool(b)),
                None => Err(Error::unexpected_input_type("bool", path))
            }
            FieldType::I32 => match json_value.as_i64() {
                Some(i) => Ok(Value::I32(i as i32)),
                None => Err(Error::unexpected_input_type("32 bit integer", path))
            }
            FieldType::I64 => match json_value.as_i64() {
                Some(i) => Ok(Value::I64(i as i64)),
                None => Err(Error::unexpected_input_type("64 bit integer", path))
            }
            FieldType::F32 => match json_value.as_f64() {
                Some(f) => Ok(Value::F32(f as f32)),
                None => Err(Error::unexpected_input_type("32 bit float", path))
            }
            FieldType::F64 => match json_value.as_f64() {
                Some(f) => Ok(Value::F64(f)),
                None => Err(Error::unexpected_input_type("64 bit float", path))
            }
            FieldType::Decimal => match json_value.as_str() {
                Some(s) => match Decimal::from_str(s) {
                    Ok(d) => Ok(Value::Decimal(d)),
                    Err(_) => Err(Error::unexpected_input_value("decimal string or float", path))
                }
                None => match json_value.as_f64() {
                    Some(f) => Ok(Value::Decimal(Decimal::from_f64_retain(f).unwrap())),
                    None => Err(Error::unexpected_input_value("decimal string or float", path))
                }
            }
            FieldType::String => match json_value.as_str() {
                Some(s) => Ok(Value::String(s.to_string())),
                None => Err(Error::unexpected_input_value("string", path))
            }
            FieldType::Date => match json_value.as_str() {
                Some(s) => match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                    Ok(naive_date) => Ok(Value::Date(naive_date)),
                    Err(_) => Err(Error::unexpected_input_value("date string", path))
                }
                None => Err(Error::unexpected_input_type("date string", path))
            }
            FieldType::DateTime => match json_value.as_str() {
                Some(s) => match DateTime::parse_from_rfc3339(s) {
                    Ok(fixed_offset_datetime) => Ok(Value::DateTime(fixed_offset_datetime.with_timezone(&Utc))),
                    Err(_) => Err(Error::unexpected_input_value("datetime string", path))
                }
                None => Err(Error::unexpected_input_type("datetime string", path))
            }
            FieldType::Enum(enum_name) => match json_value.as_str() {
                Some(s) => if graph.enum_values(enum_name.as_str()).unwrap().contains(&s.to_string()) {
                    Ok(Value::String(s.to_string()))
                } else {
                    Err(Error::unexpected_input_type(format!("string represents enum {enum_name}"), path))
                },
                None => Err(Error::unexpected_input_type(format!("string represents enum {enum_name}"), path))
            }
            FieldType::Vec(inner_field) => match json_value.as_array() {
                Some(a) => {
                    Ok(Value::Vec(a.iter().enumerate().map(|(i, v)| {
                        Self::decode_value_for_field_type(graph, inner_field.field_type(), inner_field.is_optional(), v, path + i)
                    }).collect::<Result<Vec<Value>>>()?))
                },
                None => Err(Error::unexpected_input_type("array", path))
            }
            FieldType::HashMap(inner_field) => match json_value.as_object() {
                Some(a) => {
                    Ok(Value::HashMap(a.iter().map(|(i, v)| {
                        Ok((i.to_string(), Self::decode_value_for_field_type(graph, inner_field.field_type(), inner_field.is_optional(), v, path + i)?))
                    }).collect::<Result<HashMap<String, Value>>>()?))
                },
                None => Err(Error::unexpected_input_type("object", path))
            }
            FieldType::BTreeMap(inner_field) => match json_value.as_object() {
                Some(a) => {
                    Ok(Value::BTreeMap(a.iter().map(|(i, v)| {
                        Ok((i.to_string(), Self::decode_value_for_field_type(graph, inner_field.field_type(), inner_field.is_optional(), v, path + i)?))
                    }).collect::<Result<BTreeMap<String, Value>>>()?))
                },
                None => Err(Error::unexpected_input_type("object", path))
            }
            FieldType::Object(_) => panic!("Object input is not implemented yet.")
        }
    }
}

static NESTED_UPDATE_INPUT_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"where", "update"}
});

static NESTED_CONNECT_OR_CREATE_INPUT_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"where", "create"}
});

static NESTED_UPSERT_INPUT_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"where", "create", "update"}
});

static NESTED_CREATE_ONE_ARG_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"create", "connect", "connectOrCreate"}
});

static NESTED_CREATE_MANY_ARG_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"create", "createMany", "connect", "connectOrCreate"}
});

static NESTED_UPDATE_ONE_ARG_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"create", "connect", "connectOrCreate", "set", "disconnect", "update", "delete"}
});

static NESTED_UPDATE_MANY_ARG_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"create", "createMany", "connect", "connectOrCreate", "set", "disconnect", "update", "updateMany", "upsert", "delete", "deleteMany"}
});
