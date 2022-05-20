use std::collections::HashMap;
use chrono::prelude::{Date, DateTime, Utc};
use chrono::SecondsFormat;
use serde_json::{Map, Number, Value as JsonValue};


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    ObjectId(String),
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    String(String),
    Date(Date<Utc>),
    DateTime(DateTime<Utc>),
    Enum(&'static str),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>)
}

impl Value {
    pub(crate) fn to_json_value(&self) -> JsonValue {
        match self {
            Value::Null => {
                JsonValue::Null
            }
            Value::ObjectId(val) => {
                JsonValue::String(val.clone())
            }
            Value::Bool(val) => {
                JsonValue::Bool(val.clone())
            }
            Value::I8(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::I16(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::I32(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::I64(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::I128(val) => {
                JsonValue::Number(Number::from(*val as i64))
            }
            Value::U8(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::U16(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::U32(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::U64(val) => {
                JsonValue::Number(Number::from(*val))
            }
            Value::U128(val) => {
                JsonValue::Number(Number::from(*val as u64))
            }
            Value::F32(val) => {
                JsonValue::Number(Number::from_f64(*val as f64).unwrap())
            }
            Value::F64(val) => {
                JsonValue::Number(Number::from_f64(*val).unwrap())
            }
            Value::String(val) => {
                JsonValue::String(val.clone())
            }
            Value::Date(val) => {
                JsonValue::String(val.format("%Y-%m-%d").to_string())
            }
            Value::DateTime(val) => {
                JsonValue::String(val.to_rfc3339_opts(SecondsFormat::Millis, true))
            }
            Value::Enum(val) => {
                JsonValue::String(val.to_string())
            }
            Value::Vec(val) => {
                JsonValue::Array(val.iter().map(|i| { i.to_json_value() }).collect())
            }
            Value::Map(val) => {
                let mut map = Map::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v.to_json_value());
                }
                JsonValue::Object(map)
            }
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}
