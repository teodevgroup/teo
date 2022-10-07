use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use chrono::{Date, DateTime, Utc};
use rust_decimal::Decimal;
use crate::core::tson::Value;
use crate::prelude::Object;

// MARK: - Self
impl From<&Value> for Value {
    fn from(v: &Value) -> Self {
        v.to_owned()
    }
}

// MARK: - String

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

impl<'a> From<Value> for &'a str {
    fn from(v: Value) -> &'a str {
        v.as_str().unwrap()
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        v.as_str().unwrap().to_string()
    }
}

// MARK: - Bool

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<Value> for bool {
    fn from(v: Value) -> Self {
        v.as_bool().unwrap()
    }
}

// MARK: - Numbers

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

impl From<usize> for Value {
    fn from(v: usize) -> Self { Value::U64(v as u64) }
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

impl From<Value> for Decimal {
    fn from(v: Value) -> Self {
        v.as_decimal().unwrap()
    }
}

// MARK: - Date

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

impl From<Value> for Date<Utc> {
    fn from(v: Value) -> Self {
        v.as_date().unwrap().to_owned()
    }
}

impl From<Value> for DateTime<Utc> {
    fn from(v: Value) -> Self {
        v.as_datetime().unwrap().to_owned()
    }
}

// MARK: - Collections

impl<T> From<HashMap<String, T>> for Value where T: Into<Value> {
    fn from(value: HashMap<String, T>) -> Self {
        let mut retval = HashMap::new();
        for (k, v) in value {
            retval.insert(k.to_owned(), v.into());
        }
        Value::HashMap(retval)
    }
}

impl<T> From<Value> for Vec<T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_vec().unwrap();
        let mut result: Vec<T> = vec![];
        for v in value {
            result.push(v.clone().into());
        }
        result
    }
}

impl<T> From<Value> for HashMap<String, T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: HashMap<String, T> = HashMap::new();
        for (k, v) in value {
            result.insert(k.to_owned(), (*v).into());
        }
        result
    }
}

impl<T> From<Value> for BTreeMap<String, T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: BTreeMap<String, T> = BTreeMap::new();
        for (k, v) in value {
            result.insert(k.to_owned(), (*v).into());
        }
        result
    }
}

impl<T> From<Value> for HashSet<T> where T: From<Value> + Hash + Eq + Ord {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: HashSet<T> = HashSet::new();
        for (k, v) in value {
            result.insert((*v).into());
        }
        result
    }
}

impl<T> From<Value> for BTreeSet<T> where T: From<Value> + Hash + Eq + Ord {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: BTreeSet<T> = BTreeSet::new();
        for (k, v) in value {
            result.insert((*v).into());
        }
        result
    }
}

// MARK: - Option

impl From<Value> for Option<bool> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<String> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<i8> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<i16> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<i32> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<i64> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<i128> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<u8> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<u16> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<u32> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<u64> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<u128> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<f32> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Option<f64> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<Value> for Object {
    fn from(v: Value) -> Self {
        match v {
            Value::Object(o) => o.clone(),
            _ => panic!("not object value")
        }
    }
}

impl From<Option<Object>> for Value {
    fn from(object: Option<Object>) -> Self {
        match object {
            None => Value::Null,
            Some(object) => Value::Object(object)
        }
    }
}

impl From<Value> for Option<Object> {
    fn from(v: Value) -> Self {
        match v {
            Value::Object(o) => Some(o.clone()),
            _ => None,
        }
    }
}
