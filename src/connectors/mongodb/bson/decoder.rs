use std::collections::{BTreeMap, HashMap};
use bson::Bson;
use key_path::KeyPath;

use crate::core::error::ActionError;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Value};

pub(crate) struct BsonDecoder { }

impl BsonDecoder {
    pub(crate) fn decode<'a>(model: &Model, graph: &Graph, r#type: &FieldType, optional: bool, bson_value: &Bson, path: impl AsRef<KeyPath<'a>>) -> ActionResult<Value> {
        if bson_value.as_null().is_some() && optional {
            return Ok(Value::Null);
        }
        let path = path.as_ref();
        match r#type {
            FieldType::ObjectId => match bson_value.as_object_id() {
                Some(oid) => Ok(Value::ObjectId(oid)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "object id")),
            }
            FieldType::Bool => match bson_value.as_bool() {
                Some(b) => Ok(Value::Bool(b)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "bool")),
            }
            FieldType::I32 => match bson_value.as_i32() {
                Some(n) => Ok(Value::I32(n)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "int 32")),
            }
            FieldType::I64 => match bson_value.as_i64() {
                Some(n) => Ok(Value::I64(n)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "int 64")),
            }
            FieldType::F32 => match bson_value.as_f64() {
                Some(n) => Ok(Value::F32(n as f32)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "double")),
            }
            FieldType::F64 => match bson_value.as_f64() {
                Some(n) => Ok(Value::F64(n)),
                None => Err(ActionError::record_decoding_error(model.name(), path, "double")),
            }
            FieldType::Decimal => panic!("Decimal is not implemented by MongoDB."),
            FieldType::String => match bson_value.as_str() {
                Some(s) => Ok(Value::String(s.to_owned())),
                None => Err(ActionError::record_decoding_error(model.name(), path, "string")),
            }
            FieldType::Date => match bson_value.as_datetime() {
                Some(val) => Ok(Value::Date(val.to_chrono().date_naive())),
                None => Err(ActionError::record_decoding_error(model.name(), path, "datetime")),
            }
            FieldType::DateTime => match bson_value.as_datetime() {
                Some(val) => Ok(Value::DateTime(val.to_chrono())),
                None => Err(ActionError::record_decoding_error(model.name(), path, "datetime")),
            }
            FieldType::Enum(enum_name) => match bson_value.as_str() {
                Some(val) => {
                    if graph.enum_values(enum_name).unwrap().contains(&val.to_string()) {
                        Ok(Value::String(val.to_owned()))
                    } else {
                        Err(ActionError::record_decoding_error(model.name(), path, format!("string value for enum `{enum_name}'")))
                    }
                },
                None => Err(ActionError::record_decoding_error(model.name(), path, "string")),
            }
            FieldType::Vec(inner_field) => {
                match bson_value.as_array() {
                    Some(arr) => Ok(Value::Vec(arr.iter().enumerate().map(|(i, v)| {
                        let path = path + i;
                        Self::decode(model, graph, inner_field.r#type(), inner_field.is_optional(), v, path)
                    }).collect::<ActionResult<Vec<Value>>>()?)),
                    None => Err(ActionError::record_decoding_error(model.name(), path, "array")),
                }
            }
            FieldType::HashMap(inner_field) => {
                match bson_value.as_document() {
                    Some(doc) => Ok(Value::HashMap(doc.iter().map(|(k, v)| {
                        let path = path + k;
                        Ok((k.to_owned(), Self::decode(model, graph, inner_field.r#type(), inner_field.is_optional(), v, path)?))
                    }).collect::<ActionResult<HashMap<String, Value>>>()?)),
                    None => Err(ActionError::record_decoding_error(model.name(), path, "document")),
                }
            }
            FieldType::BTreeMap(inner_field) => {
                match bson_value.as_document() {
                    Some(doc) => Ok(Value::BTreeMap(doc.iter().map(|(k, v)| {
                        let path = path + k;
                        Ok((k.to_owned(), Self::decode(model, graph, inner_field.r#type(), inner_field.is_optional(), v, path)?))
                    }).collect::<ActionResult<BTreeMap<String, Value>>>()?)),
                    None => Err(ActionError::record_decoding_error(model.name(), path, "document")),
                }
            }
            FieldType::Object(_) => panic!("Saving embedded object into database is not implemented yet.")
        }
    }
}
