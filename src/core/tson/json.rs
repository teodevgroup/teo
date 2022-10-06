use chrono::SecondsFormat;
use serde_json::{Value as JsonValue, Number as JsonNumber, Map as JsonMap};
use crate::core::tson::Value;

impl Into<JsonValue> for Value {
    fn into(self) -> Value {
        match self {
            Value::Null => {
                JsonValue::Null
            }
            #[cfg(feature = "data-source-mongodb")]
            Value::ObjectId(val) => {
                JsonValue::String(val.to_hex())
            }
            Value::Bool(val) => {
                JsonValue::Bool(val.clone())
            }
            Value::I8(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::I16(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::I32(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::I64(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::I128(val) => {
                JsonValue::Number(JsonNumber::from(*val as i64))
            }
            Value::U8(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::U16(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::U32(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::U64(val) => {
                JsonValue::Number(JsonNumber::from(*val))
            }
            Value::U128(val) => {
                JsonValue::Number(JsonNumber::from(*val as u64))
            }
            Value::F32(val) => {
                JsonValue::Number(JsonNumber::from_f64(*val as f64).unwrap())
            }
            Value::F64(val) => {
                JsonValue::Number(JsonNumber::from_f64(*val).unwrap())
            }
            Value::Decimal(val) => {
                JsonValue::String(val.to_string())
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
            Value::Vec(val) => {
                JsonValue::Array(val.iter().map(|i| { i }).collect())
            }
            Value::HashMap(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v);
                }
                JsonValue::Object(map)
            }
            Value::BTreeMap(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v);
                }
                JsonValue::Object(map)
            }
            Value::HashSet(val) => {
                JsonValue::Array(val.iter().map(|i| { i }).collect())
            }
            Value::BTreeSet(val) => {
                JsonValue::Array(val.iter().map(|i| { i }).collect())
            }
            Value::Object(_obj) => {
                panic!("Cannot convert object into json. Use specific method instead.")
            }
        }
    }
}
