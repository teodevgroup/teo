use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::{BigDecimal, FromPrimitive};
use bson::oid::ObjectId;
use chrono::{NaiveDate, Utc, DateTime};
//use itertools::Itertools;
use key_path::KeyPath;
use serde_json::Value as JsonValue;
use crate::core::interface::{ResolvedInterfaceField, ResolvedInterfaceFieldType};
use teo_teon::file::TeonFile;
use crate::prelude::{Value, Result, Error};

pub(crate) fn transform_custom_action_json_into_teon(json_value: &JsonValue, rule: &ResolvedInterfaceField, path: &KeyPath<'_>) -> Result<Value> {
    if json_value.is_null() {
        if rule.optional {
            return Ok(Value::Null);
        } else {
            return Err(Error::unexpected_input_value("required", path));
        }
    }
    validate_json_value_type(json_value, rule, path)?;
    match &rule.field_type {
        ResolvedInterfaceFieldType::Any => Ok(Value::from(json_value)),
        ResolvedInterfaceFieldType::ObjectId => match ObjectId::parse_str(json_value.as_str().unwrap()) {
            Ok(s) => Ok(Value::ObjectId(s)),
            Err(_err) => Err(Error::unexpected_input_value("object id string", path)),
        }
        ResolvedInterfaceFieldType::Bool => Ok(Value::Bool(json_value.as_bool().unwrap())),
        ResolvedInterfaceFieldType::I32 => Ok(Value::I32(json_value.as_i64().unwrap() as i32)),
        ResolvedInterfaceFieldType::I64 => Ok(Value::I64(json_value.as_i64().unwrap() as i64)),
        ResolvedInterfaceFieldType::F32 => Ok(Value::F32(json_value.as_f64().unwrap() as f32)),
        ResolvedInterfaceFieldType::F64 => Ok(Value::F32(json_value.as_f64().unwrap() as f32)),
        ResolvedInterfaceFieldType::Decimal => if json_value.is_string() {
            match BigDecimal::from_str(json_value.as_str().unwrap()) {
                Ok(d) => Ok(Value::Decimal(d)),
                Err(_) => Err(Error::unexpected_input_value("decimal string", path)),
            }
        } else if json_value.is_number() {
            Ok(Value::Decimal(BigDecimal::from_f64(json_value.as_f64().unwrap()).unwrap()))
        } else {
            unreachable!()
        }
        ResolvedInterfaceFieldType::String => Ok(Value::String(json_value.as_str().unwrap().to_owned())),
        ResolvedInterfaceFieldType::Date => match NaiveDate::parse_from_str(json_value.as_str().unwrap(), "%Y-%m-%d") {
            Ok(d) => Ok(Value::Date(d)),
            Err(_) => Err(Error::unexpected_input_value("date string", path)),
        }
        ResolvedInterfaceFieldType::DateTime => match DateTime::parse_from_rfc3339(json_value.as_str().unwrap()) {
            Ok(fixed_offset_datetime) => Ok(Value::DateTime(fixed_offset_datetime.with_timezone(&Utc))),
            Err(_) => Err(Error::unexpected_input_value("datetime string", path)),
        }
        ResolvedInterfaceFieldType::File => Ok(Value::File(TeonFile::from_json_value(json_value))),
        ResolvedInterfaceFieldType::Enum(e) => {
            let value = json_value.as_str().unwrap().to_owned();
            if e.values.contains(&value) {
                Ok(Value::String(value))
            } else {
                Err(Error::unexpected_input_value(format!("{}", e.name), path))
            }
        }
        ResolvedInterfaceFieldType::Vec(inner) => Ok(Value::Vec(json_value.as_array().unwrap().iter().enumerate().map(|(index, json_value)| {
            transform_custom_action_json_into_teon(json_value, inner.as_ref(), &(path + index))
        }).collect::<Result<Vec<Value>>>()?)),
        ResolvedInterfaceFieldType::HashMap(inner) => Ok(Value::Dictionary(json_value.as_object().unwrap().iter().map(|(key, json_value)| {
            Ok((key.clone(), transform_custom_action_json_into_teon(json_value, inner.as_ref(), &(path + key.as_str()))?))
        }).collect::<Result<HashMap<String, Value>>>()?)),
        ResolvedInterfaceFieldType::Shape(definition) => {
            let json_value_keys: Vec<&String> = json_value.as_object().unwrap().keys().collect();
            for key in json_value_keys {
                if !definition.contains_key(key) {
                    return Err(Error::unexpected_input_key(key, &(path + key.as_str())));
                }
            }
            let mut result = HashMap::<String, Value>::new();
            for (key, field) in definition {
                let item_path = &(path + key);
                let json_item = json_value.get(key);
                if field.optional && json_item.is_none() {
                    continue
                }
                if let Some(json_item) = json_item {
                    result.insert(key.clone(), transform_custom_action_json_into_teon(json_item, field, item_path)?);
                } else {
                    return Err(Error::missing_required_input(item_path));
                }
            }
            Ok(Value::Dictionary(result))
        }
    }
}

fn validate_json_value_type(json_value: &JsonValue, rule: &ResolvedInterfaceField, path: &KeyPath<'_>) -> Result<()> {
    match &rule.field_type {
        ResolvedInterfaceFieldType::File => (),
        ResolvedInterfaceFieldType::Any => (),
        ResolvedInterfaceFieldType::ObjectId => {
            if !json_value.is_string() {
                return Err(Error::unexpected_input_type("object id string", path));
            }
        }
        ResolvedInterfaceFieldType::Bool => {
            if !json_value.is_boolean() {
                return Err(Error::unexpected_input_type("bool", path));
            }
        }
        ResolvedInterfaceFieldType::I32 | ResolvedInterfaceFieldType::I64 => {
            if !(json_value.is_i64() || json_value.is_u64()) {
                return Err(Error::unexpected_input_type("integer", path));
            }
        }
        ResolvedInterfaceFieldType::F32 | ResolvedInterfaceFieldType::F64 => {
            if !json_value.is_number() {
                return Err(Error::unexpected_input_type("float", path));
            }
        }
        ResolvedInterfaceFieldType::Decimal => {
            if !(json_value.is_number() || json_value.is_string()) {
                return Err(Error::unexpected_input_type("decimal", path));
            }
        }
        ResolvedInterfaceFieldType::String => {
            if !json_value.is_string() {
                return Err(Error::unexpected_input_type("string", path));
            }
        }
        ResolvedInterfaceFieldType::Date => {
            if !json_value.is_string() {
                return Err(Error::unexpected_input_type("date string", path));
            }
        }
        ResolvedInterfaceFieldType::DateTime => {
            if !json_value.is_string() {
                return Err(Error::unexpected_input_type("datetime string", path));
            }
        }
        ResolvedInterfaceFieldType::Enum(_) => {
            if !json_value.is_string() {
                return Err(Error::unexpected_input_type("enum string", path));
            }
        }
        ResolvedInterfaceFieldType::Vec(_) => {
            if !json_value.is_array() {
                return Err(Error::unexpected_input_type("array", path));
            }
        }
        ResolvedInterfaceFieldType::HashMap(_) | ResolvedInterfaceFieldType::Shape(_) => {
            if !json_value.is_object() {
                return Err(Error::unexpected_input_type("object", path));
            }
        }
    }
    Ok(())
}