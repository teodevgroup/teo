#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use bigdecimal::BigDecimal;
use bson::oid::ObjectId;
use chrono::{DateTime, NaiveDate, Utc};
use teo::prelude::{Value, Result, Error, File};

pub trait Interface {

    fn inner(&self) -> &Value;

    fn inner_mut(&mut self) -> &mut Value;
}

pub trait AsInterface where Self: Sized {

    fn from_value(value: Value) -> Result<Self>;
}

pub trait AsInterfaceRef where Self: Sized {

    fn from_value_ref(value: &Value) -> Result<&Self>;
}

pub trait AsInterfaceVecRef<T> where Self: Sized {
    fn from_value_ref_vec(value: &Value) -> Result<Vec<&T>>;
}

impl AsInterface for Value {
    fn from_value(value: Value) -> Result<Self> {
        Ok(value)
    }
}

impl AsInterfaceRef for Value {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        Ok(value)
    }
}

impl AsInterface for bool {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Bool(i) => Ok(i),
            _ => Err(Error::new("value is not bool")),
        }
    }
}

impl AsInterfaceRef for bool {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Bool(i) => Ok(i),
            _ => Err(Error::new("value is not bool")),
        }
    }
}

impl AsInterface for String {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::String(i) => Ok(i),
            _ => Err(Error::new("value is not String")),
        }
    }
}

impl AsInterfaceRef for String {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::String(i) => Ok(i),
            _ => Err(Error::new("value is not String")),
        }
    }
}

impl AsInterface for i32 {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(Error::new("value is not i32")),
        }
    }
}

impl AsInterfaceRef for i32 {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(Error::new("value is not i32")),
        }
    }
}

impl AsInterface for i64 {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(Error::new("value is not i64")),
        }
    }
}

impl AsInterfaceRef for i64 {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(Error::new("value is not i64")),
        }
    }
}

impl AsInterface for f32 {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Float32(i) => Ok(i),
            _ => Err(Error::new("value is not f32")),
        }
    }
}

impl AsInterfaceRef for f32 {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Float32(i) => Ok(i),
            _ => Err(Error::new("value is not f32")),
        }
    }
}

impl AsInterface for f64 {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Float(i) => Ok(i),
            _ => Err(Error::new("value is not f64")),
        }
    }
}

impl AsInterfaceRef for f64 {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Float(i) => Ok(i),
            _ => Err(Error::new("value is not f64")),
        }
    }
}

impl AsInterface for BigDecimal {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Decimal(i) => Ok(i),
            _ => Err(Error::new("value is not BigDecimal")),
        }
    }
}

impl AsInterfaceRef for BigDecimal {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Decimal(i) => Ok(i),
            _ => Err(Error::new("value is not BigDecimal")),
        }
    }
}

impl AsInterface for ObjectId {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::ObjectId(i) => Ok(i),
            _ => Err(Error::new("value is not ObjectId")),
        }
    }
}

impl AsInterfaceRef for ObjectId {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::ObjectId(i) => Ok(i),
            _ => Err(Error::new("value is not ObjectId")),
        }
    }
}

impl AsInterface for NaiveDate {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Date(i) => Ok(i),
            _ => Err(Error::new("value is not NaiveDate")),
        }
    }
}

impl AsInterfaceRef for NaiveDate {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::Date(i) => Ok(i),
            _ => Err(Error::new("value is not NaiveDate")),
        }
    }
}

impl AsInterface for DateTime<Utc> {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::DateTime(i) => Ok(i),
            _ => Err(Error::new("value is not DateTime<Utc>")),
        }
    }
}

impl AsInterfaceRef for DateTime<Utc> {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::DateTime(i) => Ok(i),
            _ => Err(Error::new("value is not DateTime<Utc>")),
        }
    }
}

impl<T> AsInterface for Vec<T> where T: AsInterface {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::Array(v) => Ok(v.into_iter().map(<T as AsInterface>::from_value).collect::<Result<Vec<T>>>()?),
            _ => Err(Error::new("value is not Vec<T>")),
        }
    }
}

impl<T> AsInterfaceVecRef<T> for Vec<&T> where T: AsInterfaceRef {
    fn from_value_ref_vec(value: &Value) -> Result<Vec<&T>> {
        match value {
            Value::Array(v) => Ok(v.iter().map(<T as AsInterfaceRef>::from_value_ref).collect::<Result<Vec<&T>>>()?),
            _ => Err(Error::new("value is not Vec<&T>")),
        }
    }
}

impl AsInterface for File {
    fn from_value(value: Value) -> Result<Self> {
        match value {
            Value::File(i) => Ok(i),
            _ => Err(Error::new("value is not File")),
        }
    }
}

impl AsInterfaceRef for File {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        match value {
            Value::File(i) => Ok(i),
            _ => Err(Error::new("value is not File")),
        }
    }
}
