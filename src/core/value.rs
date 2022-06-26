use std::collections::HashMap;
use std::ops::Neg;
use chrono::prelude::{Date, DateTime, Utc};
use chrono::SecondsFormat;
use rust_decimal::Decimal;
use serde_json::{Map, Number, Value as JsonValue};
use crate::core::object::Object;


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
    Decimal(Decimal),
    String(String),
    Date(Date<Utc>),
    DateTime(DateTime<Utc>),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>),
    Object(Object)
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
                JsonValue::Array(val.iter().map(|i| { i.to_json_value() }).collect())
            }
            Value::Map(val) => {
                let mut map = Map::new();
                for (k, v) in val {
                    map.insert(k.to_string(), v.to_json_value());
                }
                JsonValue::Object(map)
            }
            Value::Object(obj) => {
                obj.to_json()
            }
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v.as_str()),
            Value::ObjectId(v) => Some(v.as_str()),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(v) => Some(v.clone()),
            Value::ObjectId(v) => Some(v.clone()),
            _ => None
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Value::I8(v) => Some(*v),
            Value::I16(v) => Some(*v as i8),
            Value::I32(v) => Some(*v as i8),
            Value::I64(v) => Some(*v as i8),
            Value::I128(v) => Some(*v as i8),
            Value::U8(v) => Some(*v as i8),
            Value::U16(v) => Some(*v as i8),
            Value::U32(v) => Some(*v as i8),
            Value::U64(v) => Some(*v as i8),
            Value::U128(v) => Some(*v as i8),
            _ => None
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Value::I8(v) => Some(*v as i16),
            Value::I16(v) => Some(*v),
            Value::I32(v) => Some(*v as i16),
            Value::I64(v) => Some(*v as i16),
            Value::I128(v) => Some(*v as i16),
            Value::U8(v) => Some(*v as i16),
            Value::U16(v) => Some(*v as i16),
            Value::U32(v) => Some(*v as i16),
            Value::U64(v) => Some(*v as i16),
            Value::U128(v) => Some(*v as i16),
            _ => None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I8(v) => Some(*v as i32),
            Value::I16(v) => Some(*v as i32),
            Value::I32(v) => Some(*v),
            Value::I64(v) => Some(*v as i32),
            Value::I128(v) => Some(*v as i32),
            Value::U8(v) => Some(*v as i32),
            Value::U16(v) => Some(*v as i32),
            Value::U32(v) => Some(*v as i32),
            Value::U64(v) => Some(*v as i32),
            Value::U128(v) => Some(*v as i32),
            _ => None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I8(v) => Some(*v as i64),
            Value::I16(v) => Some(*v as i64),
            Value::I32(v) => Some(*v as i64),
            Value::I64(v) => Some(*v),
            Value::I128(v) => Some(*v as i64),
            Value::U8(v) => Some(*v as i64),
            Value::U16(v) => Some(*v as i64),
            Value::U32(v) => Some(*v as i64),
            Value::U64(v) => Some(*v as i64),
            Value::U128(v) => Some(*v as i64),
            _ => None
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Value::I8(v) => Some(*v as i128),
            Value::I16(v) => Some(*v as i128),
            Value::I32(v) => Some(*v as i128),
            Value::I64(v) => Some(*v as i128),
            Value::I128(v) => Some(*v),
            Value::U8(v) => Some(*v as i128),
            Value::U16(v) => Some(*v as i128),
            Value::U32(v) => Some(*v as i128),
            Value::U64(v) => Some(*v as i128),
            Value::U128(v) => Some(*v as i128),
            _ => None
        }
    }

    //
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Value::U8(v) => Some(*v as u8),
            Value::U16(v) => Some(*v as u8),
            Value::U32(v) => Some(*v as u8),
            Value::U64(v) => Some(*v as u8),
            Value::U128(v) => Some(*v as u8),
            _ => None
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Value::U8(v) => Some(*v as u16),
            Value::U16(v) => Some(*v as u16),
            Value::U32(v) => Some(*v as u16),
            Value::U64(v) => Some(*v as u16),
            Value::U128(v) => Some(*v as u16),
            _ => None
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::U8(v) => Some(*v as u32),
            Value::U16(v) => Some(*v as u32),
            Value::U32(v) => Some(*v as u32),
            Value::U64(v) => Some(*v as u32),
            Value::U128(v) => Some(*v as u32),
            _ => None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U8(v) => Some(*v as u64),
            Value::U16(v) => Some(*v as u64),
            Value::U32(v) => Some(*v as u64),
            Value::U64(v) => Some(*v as u64),
            Value::U128(v) => Some(*v as u64),
            _ => None
        }
    }

    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Value::U8(v) => Some(*v as u128),
            Value::U16(v) => Some(*v as u128),
            Value::U32(v) => Some(*v as u128),
            Value::U64(v) => Some(*v as u128),
            Value::U128(v) => Some(*v as u128),
            _ => None
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::I8(v) => Some(*v as f32),
            Value::I16(v) => Some(*v as f32),
            Value::I32(v) => Some(*v as f32),
            Value::I64(v) => Some(*v as f32),
            Value::I128(v) => Some(*v as f32),
            Value::U8(v) => Some(*v as f32),
            Value::U16(v) => Some(*v as f32),
            Value::U32(v) => Some(*v as f32),
            Value::U64(v) => Some(*v as f32),
            Value::U128(v) => Some(*v as f32),
            Value::F32(v) => Some(*v),
            Value::F64(v) => Some(*v as f32),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::I8(v) => Some(*v as f64),
            Value::I16(v) => Some(*v as f64),
            Value::I32(v) => Some(*v as f64),
            Value::I64(v) => Some(*v as f64),
            Value::I128(v) => Some(*v as f64),
            Value::U8(v) => Some(*v as f64),
            Value::U16(v) => Some(*v as f64),
            Value::U32(v) => Some(*v as f64),
            Value::U64(v) => Some(*v as f64),
            Value::U128(v) => Some(*v as f64),
            Value::F32(v) => Some(*v as f64),
            Value::F64(v) => Some(*v as f64),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(val) => Some(val),
            _ => None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Value::I8(n) => Some(*n as usize),
            Value::I16(n) => Some(*n as usize),
            Value::I32(n) => Some(*n as usize),
            Value::I64(n) => Some(*n as usize),
            Value::I128(n) => Some(*n as usize),
            Value::U8(n) => Some(*n as usize),
            Value::U16(n) => Some(*n as usize),
            Value::U32(n) => Some(*n as usize),
            Value::U64(n) => Some(*n as usize),
            Value::U128(n) => Some(*n as usize),
            _ => None
        }
    }

    pub(crate) fn recip(&self) -> f64 {
        match self {
            Value::I8(n) => (*n as f64).recip(),
            Value::I16(n) => (*n as f64).recip(),
            Value::I32(n) => (*n as f64).recip(),
            Value::I64(n) => (*n as f64).recip(),
            Value::I128(n) => (*n as f64).recip(),
            Value::U8(n) => (*n as f64).recip(),
            Value::U16(n) => (*n as f64).recip(),
            Value::U32(n) => (*n as f64).recip(),
            Value::U64(n) => (*n as f64).recip(),
            Value::U128(n) => (*n as f64).recip(),
            Value::F32(n) => (*n as f64).recip(),
            Value::F64(n) => (*n as f64).recip(),
            Value::Decimal(n) => panic!("decimal div todo"),
            _ => panic!()
        }
    }

    pub(crate) fn neg(&self) -> Value {
        match self {
            Value::Bool(val) => {
                Value::Bool(if *val { false } else { true })
            }
            Value::I8(val) => {
                Value::I8(-*val)
            }
            Value::I16(val) => {
                Value::I16(-*val)
            }
            Value::I32(val) => {
                Value::I32(-*val)
            }
            Value::I64(val) => {
                Value::I64(-*val)
            }
            Value::I128(val) => {
                Value::I128(-*val)
            }
            Value::F32(val) => {
                Value::F32(-*val)
            }
            Value::F64(val) => {
                Value::F64(-*val)
            }
            Value::Decimal(val) => {
                Value::Decimal(-*val)
            }
            Value::U8(val) => {
                Value::I8(-(*val as i8))
            }
            Value::U16(val) => {
                Value::I16(-(*val as i16))
            }
            Value::U32(val) => {
                Value::I32(-(*val as i32))
            }
            Value::U64(val) => {
                Value::I64(-(*val as i64))
            }
            Value::U128(val) => {
                Value::I128(-(*val as i128))
            }
            _ => {
                panic!("Cannot neg.")
            }
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::I8(v)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::I16(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::I32(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::I64(v)
    }
}

impl From<i128> for Value {
    fn from(v: i128) -> Self {
        Value::I128(v)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value::U8(v)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::U16(v)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::U32(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::U64(v)
    }
}

impl From<u128> for Value {
    fn from(v: u128) -> Self {
        Value::U128(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::F32(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self { Value::Decimal(v) }
}

impl From<Date<Utc>> for Value {
    fn from(v: Date<Utc>) -> Self {
        Value::Date(v)
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(v: DateTime<Utc>) -> Self {
        Value::DateTime(v)
    }
}

// new

impl<'a> From<&'a Value> for &'a str {
    fn from(v: &'a Value) -> Self {
        v.as_str().unwrap()
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        v.as_string().unwrap()
    }
}

impl From<Value> for bool {
    fn from(v: Value) -> Self {
        v.as_bool().unwrap()
    }
}

impl From<Value> for i8 {
    fn from(v: Value) -> Self {
        v.as_i8().unwrap()
    }
}

impl From<Value> for i16 {
    fn from(v: Value) -> Self {
        v.as_i16().unwrap()
    }
}


impl From<Value> for i32 {
    fn from(v: Value) -> Self {
        v.as_i32().unwrap()
    }
}


impl From<Value> for i64 {
    fn from(v: Value) -> Self {
        v.as_i64().unwrap()
    }
}

impl From<Value> for i128 {
    fn from(v: Value) -> Self {
        v.as_i128().unwrap()
    }
}

impl From<Value> for u8 {
    fn from(v: Value) -> Self {
        v.as_u8().unwrap()
    }
}


impl From<Value> for u16 {
    fn from(v: Value) -> Self {
        v.as_u16().unwrap()
    }
}

impl From<Value> for u32 {
    fn from(v: Value) -> Self {
        v.as_u32().unwrap()
    }
}


impl From<Value> for u64 {
    fn from(v: Value) -> Self {
        v.as_u64().unwrap()
    }
}

impl From<Value> for u128 {
    fn from(v: Value) -> Self {
        v.as_u128().unwrap()
    }
}


impl From<Value> for f32 {
    fn from(v: Value) -> Self {
        v.as_f32().unwrap()
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        v.as_f64().unwrap()
    }
}
//
// impl From<Decimal> for Value {
//     fn from(v: Decimal) -> Self { Value::Decimal(v) }
// }
//
// impl From<Date<Utc>> for Value {
//     fn from(v: Date<Utc>) -> Self {
//         Value::Date(v)
//     }
// }
//
// impl From<DateTime<Utc>> for Value {
//     fn from(v: DateTime<Utc>) -> Self {
//         Value::DateTime(v)
//     }
// }
