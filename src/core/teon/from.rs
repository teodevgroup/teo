use std::collections::{BTreeMap, HashMap};
use bson::oid::ObjectId;
use chrono::{NaiveDate, DateTime, Utc};
use bigdecimal::BigDecimal;
use crate::core::teon::Value;
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

impl<'a> From<&'a Value> for &'a str {
    fn from(v: &'a Value) -> &'a str {
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

impl From<BigDecimal> for Value {
    fn from(v: BigDecimal) -> Self { Value::Decimal(v) }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self { Value::I64(v as i64) }
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

impl From<Value> for BigDecimal {
    fn from(v: Value) -> Self {
        v.as_decimal().unwrap()
    }
}

// MARK: - Date

impl From<NaiveDate> for Value {
    fn from(v: NaiveDate) -> Self {
        Value::Date(v)
    }
}

impl From<Value> for NaiveDate {
    fn from(v: Value) -> Self {
        v.as_date().unwrap().to_owned()
    }
}

impl From<Option<NaiveDate>> for Value {
    fn from(n: Option<NaiveDate>) -> Self {
        match n {
            Some(n) => Value::Date(n),
            None => Value::Null,
        }
    }
}

impl From<Value> for Option<NaiveDate> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(v: DateTime<Utc>) -> Self {
        Value::DateTime(v)
    }
}

impl From<Value> for DateTime<Utc> {
    fn from(v: Value) -> Self {
        v.as_datetime().unwrap().to_owned()
    }
}

impl From<Option<DateTime<Utc>>> for Value {
    fn from(n: Option<DateTime<Utc>>) -> Self {
        match n {
            Some(n) => Value::DateTime(n),
            None => Value::Null,
        }
    }
}

impl From<Value> for Option<DateTime<Utc>> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            _ => Some(value.into())
        }
    }
}

// MARK: ObjectID

#[cfg(feature = "data-source-mongodb")]
impl From<Value> for ObjectId {
    fn from(value: Value) -> Self {
        value.as_object_id().unwrap().clone()
    }
}

#[cfg(feature = "data-source-mongodb")]
impl From<Value> for Option<ObjectId> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => None,
            Value::ObjectId(o) => Some(o.clone()),
            _ => panic!(),
        }
    }
}

#[cfg(feature = "data-source-mongodb")]
impl From<ObjectId> for Value {
    fn from(value: ObjectId) -> Self {
        Value::ObjectId(value)
    }
}

#[cfg(feature = "data-source-mongodb")]
impl From<Option<ObjectId>> for Value {
    fn from(value: Option<ObjectId>) -> Self {
        match value {
            Some(o) => Value::ObjectId(o.clone()),
            None => Value::Null,
        }
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
            result.insert(k.to_owned(), (v.clone()).into());
        }
        result
    }
}

impl<T> From<Value> for BTreeMap<String, T> where T: From<Value> {
    fn from(value: Value) -> Self {
        let value = value.as_hashmap().unwrap();
        let mut result: BTreeMap<String, T> = BTreeMap::new();
        for (k, v) in value {
            result.insert(k.to_owned(), (v.clone()).into());
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

impl From<Option<bool>> for Value {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(b) => Value::Bool(b),
            None => Value::Null,
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

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Value::String(s.clone()),
            None => Value::Null,
        }
    }
}

impl From<Option<i32>> for Value {
    fn from(n: Option<i32>) -> Self {
        match n {
            Some(n) => Value::I32(n),
            None => Value::Null,
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

impl From<Option<i64>> for Value {
    fn from(n: Option<i64>) -> Self {
        match n {
            Some(n) => Value::I64(n),
            None => Value::Null,
        }
    }
}

impl From<Option<BigDecimal>> for Value {
    fn from(n: Option<BigDecimal>) -> Self {
        match n {
            Some(n) => Value::Decimal(n),
            None => Value::Null,
        }
    }
}

impl From<Value> for Option<BigDecimal> {
    fn from(value: Value) -> Self {
        match value {
            Value::Decimal(n) => Some(n),
            _ => None,
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

impl From<Option<f32>> for Value {
    fn from(n: Option<f32>) -> Self {
        match n {
            Some(n) => Value::F32(n),
            None => Value::Null,
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

impl From<Option<f64>> for Value {
    fn from(n: Option<f64>) -> Self {
        match n {
            Some(n) => Value::F64(n),
            None => Value::Null,
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
