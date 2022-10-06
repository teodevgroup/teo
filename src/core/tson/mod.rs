use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::mem;
use std::ops::{Add, Div, Mul, Sub, Rem, Neg};
use async_recursion::async_recursion;
use chrono::prelude::{Date, DateTime, Utc};
use chrono::SecondsFormat;
use rust_decimal::Decimal;
#[cfg(feature = "data-source-mongodb")]
use bson::oid::ObjectId;
use crate::core::field::r#type::FieldType;
use crate::core::object::Object;
use crate::core::tson::index::Index;

pub mod index;
pub mod json;
pub mod from;
pub mod macros;
pub(crate) mod decoder;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/mod.rs

/// Represents any valid Tson value. A Tson value is an extension for Teo just like Bson for
/// MongoDB.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {

    /// Represents a JSON null value.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(null);
    /// ```
    Null,

    /// Represents a Tson bool.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(true);
    /// ```
    Bool(bool),

    /// Represents a Tson i8.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_i8);
    /// ```
    I8(i8),

    /// Represents a Tson i16.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_i16);
    /// ```
    I16(i16),

    /// Represents a Tson i32.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_i32);
    /// ```
    I32(i32),

    /// Represents a Tson i64.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_i64);
    /// ```
    I64(i64),

    /// Represents a Tson i128.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_i128);
    /// ```
    I128(i128),

    /// Represents a Tson u8.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_u8);
    /// ```
    U8(u8),

    /// Represents a Tson u16.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_u16);
    /// ```
    U16(u16),

    /// Represents a Tson u32.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_u32);
    /// ```
    U32(u32),

    /// Represents a Tson u64.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_u64);
    /// ```
    U64(u64),

    /// Represents a Tson u128.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12_u128);
    /// ```
    U128(u128),

    /// Represents a Tson f32.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12.5_f32);
    /// ```
    F32(f32),

    /// Represents a Tson f64.
    ///
    /// ```
    /// # use teo::prelude::tson;
    /// #
    /// let v = tson!(12.5_f64);
    /// ```
    F64(f64),

    /// Represents a Tson decimal.
    ///
    Decimal(Decimal),

    /// Represents a Tson object id.
    ///
    #[cfg(feature = "data-source-mongodb")]
    ObjectId(ObjectId),

    /// Represents a Tson string.
    ///
    String(String),

    /// Represents a Tson date.
    ///
    Date(Date<Utc>),

    /// Represents a Tson datetime.
    ///
    DateTime(DateTime<Utc>),

    /// Represents a Tson array.
    ///
    Vec(Vec<Value>),

    /// Represents a Tson hashmap.
    ///
    HashMap(HashMap<String, Value>),

    /// Represents a Tson btreemap.
    ///
    BTreeMap(BTreeMap<String, Value>),

    /// Represents a Tson hashset.
    ///
    HashSet(HashSet<Value>),

    /// Represents a Tson btreeset.
    ///
    BTreeSet(BTreeSet<Value>),

    /// Represents a Tson object.
    ///
    Object(Object),
}

impl Value {

    // TODO: remove after

    pub(crate) fn number_from_f64(num: f64, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::F32 => Value::F32(num as f32),
            FieldType::F64 => Value::F64(num),
            _ => panic!()
        }
    }

    pub(crate) fn number_from_i64(num: i64, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::I8 => Value::I8(num as i8),
            FieldType::I16 => Value::I16(num as i16),
            FieldType::I32 => Value::I32(num as i32),
            FieldType::I64 => Value::I64(num as i64),
            FieldType::I128 => Value::I128(num as i128),
            FieldType::U8 => Value::U8(num as u8),
            FieldType::U16 => Value::U16(num as u16),
            FieldType::U32 => Value::U32(num as u32),
            FieldType::U64 => Value::U64(num as u64),
            FieldType::U128 => Value::U128(num as u128),
            _ => panic!()
        }
    }

    #[async_recursion]
    pub(crate) async fn to_object_json_value(&self) -> Option<Value> {
        match self {
            Value::Object(o) => {
                match o.to_json().await {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            _ => None
        }
    }

    #[async_recursion]
    pub(crate) async fn to_object_vec_json_value(&self) -> Option<Value> {
        match self {
            Value::Vec(vec) => {
                let mut result: Vec<Value> = vec![];
                for object in vec {
                    result.push(object.to_object_json_value().await.unwrap());
                }
                Some(Value::Array(result))
            }
            _ => None
        }
    }

    pub fn is_object_vec(&self) -> bool {
        match self {
            Value::Vec(v) => {
                if v.is_empty() {
                    false
                } else {
                    v.get(0).unwrap().is_object()
                }
            }
            _ => false,
        }
    }

    // TODO: remove before

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    pub fn is_hashmap(&self) -> bool {
        self.as_hashmap().is_some()
    }

    pub fn as_hashmap(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::HashMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_hashmap_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::HashMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_btreemap(&self) -> bool {
        self.as_btreemap().is_some()
    }

    pub fn as_btreemap(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::BTreeMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_btreemap_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::BTreeMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_hashset(&self) -> bool {
        self.as_hashset().is_some()
    }

    pub fn as_hashset(&self) -> Option<&HashSet<Value>> {
        match self {
            Value::HashSet(set) => Some(set),
            _ => None,
        }
    }

    pub fn as_hashset_mut(&mut self) -> Option<&mut HashSet<Value>> {
        match self {
            Value::HashSet(set) => Some(set),
            _ => None,
        }
    }

    pub fn is_btreeset(&self) -> bool {
        self.as_btreemap().is_some()
    }

    pub fn as_btreeset(&self) -> Option<&BTreeSet<Value>> {
        match self {
            Value::BTreeSet(set) => Some(set),
            _ => None,
        }
    }

    pub fn as_btreeset_mut(&mut self) -> Option<&mut BTreeSet<Value>> {
        match self {
            Value::BTreeSet(set) => Some(set),
            _ => None,
        }
    }

    pub fn is_vec(&self) -> bool {
        self.as_vec().is_some()
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Vec(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Vec(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_i(&self) -> bool {
        match *self {
            Value::I8(_) | Value::I16(_) | Value::I32(_) | Value::I64(_) | Value::I128(_) => true,
            _ => false,
        }
    }

    pub fn is_u(&self) -> bool {
        match *self {
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) | Value::U128(_) => true,
            _ => false,
        }
    }

    pub fn is_f(&self) -> bool {
        match *self {
            Value::F32(_) | Value::F64(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        self.is_i() || self.is_u() || self.is_f()
    }

    pub fn is_i8(&self) -> bool {
        match *self {
            Value::I8(_) => true,
            _ => false,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match *self {
            Value::I8(v) => Some(v),
            Value::I16(v) => Some(v as i8),
            Value::I32(v) => Some(v as i8),
            Value::I64(v) => Some(v as i8),
            Value::I128(v) => Some(v as i8),
            Value::U8(v) => Some(v as i8),
            Value::U16(v) => Some(v as i8),
            Value::U32(v) => Some(v as i8),
            Value::U64(v) => Some(v as i8),
            Value::U128(v) => Some(v as i8),
            _ => None
        }
    }

    pub fn is_i16(&self) -> bool {
        match *self {
            Value::I16(_) => true,
            _ => false,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match *self {
            Value::I8(v) => Some(v as i16),
            Value::I16(v) => Some(v),
            Value::I32(v) => Some(v as i16),
            Value::I64(v) => Some(v as i16),
            Value::I128(v) => Some(v as i16),
            Value::U8(v) => Some(v as i16),
            Value::U16(v) => Some(v as i16),
            Value::U32(v) => Some(v as i16),
            Value::U64(v) => Some(v as i16),
            Value::U128(v) => Some(v as i16),
            _ => None
        }
    }

    pub fn is_i32(&self) -> bool {
        match *self {
            Value::I32(_) => true,
            _ => false,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Value::I8(v) => Some(v as i32),
            Value::I16(v) => Some(v as i32),
            Value::I32(v) => Some(v),
            Value::I64(v) => Some(v as i32),
            Value::I128(v) => Some(v as i32),
            Value::U8(v) => Some(v as i32),
            Value::U16(v) => Some(v as i32),
            Value::U32(v) => Some(v as i32),
            Value::U64(v) => Some(v as i32),
            Value::U128(v) => Some(v as i32),
            _ => None
        }
    }

    pub fn is_i64(&self) -> bool {
        match *self {
            Value::I64(_) => true,
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::I8(v) => Some(v as i64),
            Value::I16(v) => Some(v as i64),
            Value::I32(v) => Some(v as i64),
            Value::I64(v) => Some(v),
            Value::I128(v) => Some(v as i64),
            Value::U8(v) => Some(v as i64),
            Value::U16(v) => Some(v as i64),
            Value::U32(v) => Some(v as i64),
            Value::U64(v) => Some(v as i64),
            Value::U128(v) => Some(v as i64),
            _ => None
        }
    }

    pub fn is_i128(&self) -> bool {
        match *self {
            Value::I128(_) => true,
            _ => false,
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        match *self {
            Value::I8(v) => Some(v as i128),
            Value::I16(v) => Some(v as i128),
            Value::I32(v) => Some(v as i128),
            Value::I64(v) => Some(v as i128),
            Value::I128(v) => Some(v),
            Value::U8(v) => Some(v as i128),
            Value::U16(v) => Some(v as i128),
            Value::U32(v) => Some(v as i128),
            Value::U64(v) => Some(v as i128),
            Value::U128(v) => Some(v as i128),
            _ => None
        }
    }

    pub fn is_u8(&self) -> bool {
        match *self {
            Value::U8(v) => true,
            _ => false,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Value::I8(v) => Some(*v as u8),
            Value::I16(v) => Some(*v as u8),
            Value::I32(v) => Some(*v as u8),
            Value::I64(v) => Some(*v as u8),
            Value::I128(v) => Some(*v as u8),
            Value::U8(v) => Some(*v as u8),
            Value::U16(v) => Some(*v as u8),
            Value::U32(v) => Some(*v as u8),
            Value::U64(v) => Some(*v as u8),
            Value::U128(v) => Some(*v as u8),
            _ => None
        }
    }

    pub fn is_u16(&self) -> bool {
        match *self {
            Value::U16(v) => true,
            _ => false,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Value::I8(v) => Some(*v as u16),
            Value::I16(v) => Some(*v as u16),
            Value::I32(v) => Some(*v as u16),
            Value::I64(v) => Some(*v as u16),
            Value::I128(v) => Some(*v as u16),
            Value::U8(v) => Some(*v as u16),
            Value::U16(v) => Some(*v as u16),
            Value::U32(v) => Some(*v as u16),
            Value::U64(v) => Some(*v as u16),
            Value::U128(v) => Some(*v as u16),
            _ => None
        }
    }

    pub fn is_u32(&self) -> bool {
        match *self {
            Value::U32(v) => true,
            _ => false,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Value::I8(v) => Some(*v as u32),
            Value::I16(v) => Some(*v as u32),
            Value::I32(v) => Some(*v as u32),
            Value::I64(v) => Some(*v as u32),
            Value::I128(v) => Some(*v as u32),
            Value::U8(v) => Some(*v as u32),
            Value::U16(v) => Some(*v as u32),
            Value::U32(v) => Some(*v as u32),
            Value::U64(v) => Some(*v as u32),
            Value::U128(v) => Some(*v as u32),
            _ => None
        }
    }

    pub fn is_u64(&self) -> bool {
        match *self {
            Value::U64(v) => true,
            _ => false,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::I8(v) => Some(*v as u64),
            Value::I16(v) => Some(*v as u64),
            Value::I32(v) => Some(*v as u64),
            Value::I64(v) => Some(*v as u64),
            Value::I128(v) => Some(*v as u64),
            Value::U8(v) => Some(*v as u64),
            Value::U16(v) => Some(*v as u64),
            Value::U32(v) => Some(*v as u64),
            Value::U64(v) => Some(*v as u64),
            Value::U128(v) => Some(*v as u64),
            _ => None
        }
    }

    pub fn is_u128(&self) -> bool {
        match *self {
            Value::U128(v) => true,
            _ => false,
        }
    }

    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Value::I8(v) => Some(*v as u128),
            Value::I16(v) => Some(*v as u128),
            Value::I32(v) => Some(*v as u128),
            Value::I64(v) => Some(*v as u128),
            Value::I128(v) => Some(*v as u128),
            Value::U8(v) => Some(*v as u128),
            Value::U16(v) => Some(*v as u128),
            Value::U32(v) => Some(*v as u128),
            Value::U64(v) => Some(*v as u128),
            Value::U128(v) => Some(*v as u128),
            _ => None
        }
    }

    pub fn is_f32(&self) -> bool {
        match *self {
            Value::F32(v) => true,
            _ => false,
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

    pub fn is_f64(&self) -> bool {
        match *self {
            Value::F64(v) => true,
            _ => false,
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

    pub fn is_decimal(&self) -> bool {
        match *self {
            Value::Decimal(_) => true,
            _ => false,
        }
    }

    pub fn as_decimal(&self) -> Option<Decimal> {
        match *self {
            Value::Decimal(v) => Some(v),
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

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_date(&self) -> bool {
        self.as_date().is_some()
    }

    pub fn as_date(&self) -> Option<&Date<Utc>> {
        match self {
            Value::Date(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    pub fn as_datetime(&self) -> Option<&DateTime<Utc>> {
        match self {
            Value::DateTime(d) => Some(d),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
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
            Value::Decimal(_n) => panic!("decimal div todo"),
            _ => panic!()
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::Null
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Value::*;
        match (self, other) {
            (Null, Null) => Some(Ordering::Equal),
            (ObjectId(s), ObjectId(o)) => s.partial_cmp(o),
            (Bool(s), Bool(o)) => s.partial_cmp(o),
            (I8(s), I8(o)) => s.partial_cmp(o),
            (I16(s), I16(o)) => s.partial_cmp(o),
            (I32(s), I32(o)) => s.partial_cmp(o),
            (I64(s), I64(o)) => s.partial_cmp(o),
            (I128(s), I128(o)) => s.partial_cmp(o),
            (U8(s), U8(o)) => s.partial_cmp(o),
            (U16(s), U16(o)) => s.partial_cmp(o),
            (U32(s), U32(o)) => s.partial_cmp(o),
            (U64(s), U64(o)) => s.partial_cmp(o),
            (U128(s), U128(o)) => s.partial_cmp(o),
            (F32(s), F32(o)) => s.partial_cmp(o),
            (F64(s), F64(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Vec(s), Vec(o)) => s.partial_cmp(o),
            (HashMap(_s), HashMap(_o)) => None,
            (HashSet(_s), HashSet(_o)) => None,
            (BTreeSet(_s), BTreeSet(_o)) => None,
            (BTreeMap(_s), BTreeMap(_o)) => None,
            (Object(_s), Object(_o)) => None,
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::I8(v) => Value::I8(v + rhs.as_i8().unwrap()),
            Value::I16(v) => Value::I16(v + rhs.as_i16().unwrap()),
            Value::I32(v) => Value::I32(v + rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v + rhs.as_i64().unwrap()),
            Value::I128(v) => Value::I128(v + rhs.as_i128().unwrap()),
            Value::U8(v) => Value::U8(v + rhs.as_u8().unwrap()),
            Value::U16(v) => Value::U16(v + rhs.as_u16().unwrap()),
            Value::U32(v) => Value::U32(v + rhs.as_u32().unwrap()),
            Value::U64(v) => Value::U64(v + rhs.as_u64().unwrap()),
            Value::U128(v) => Value::U128(v + rhs.as_u128().unwrap()),
            Value::F32(v) => Value::F32(v + rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v + rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d + rhs.as_decimal().unwrap()),
            _ => Value::Null,
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::I8(v) => Value::I8(v - rhs.as_i8().unwrap()),
            Value::I16(v) => Value::I16(v - rhs.as_i16().unwrap()),
            Value::I32(v) => Value::I32(v - rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v - rhs.as_i64().unwrap()),
            Value::I128(v) => Value::I128(v - rhs.as_i128().unwrap()),
            Value::U8(v) => Value::U8(v - rhs.as_u8().unwrap()),
            Value::U16(v) => Value::U16(v - rhs.as_u16().unwrap()),
            Value::U32(v) => Value::U32(v - rhs.as_u32().unwrap()),
            Value::U64(v) => Value::U64(v - rhs.as_u64().unwrap()),
            Value::U128(v) => Value::U128(v - rhs.as_u128().unwrap()),
            Value::F32(v) => Value::F32(v - rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v - rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d - rhs.as_decimal().unwrap()),
            _ => Value::Null,
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::I8(v) => Value::I8(v * rhs.as_i8().unwrap()),
            Value::I16(v) => Value::I16(v * rhs.as_i16().unwrap()),
            Value::I32(v) => Value::I32(v * rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v * rhs.as_i64().unwrap()),
            Value::I128(v) => Value::I128(v * rhs.as_i128().unwrap()),
            Value::U8(v) => Value::U8(v * rhs.as_u8().unwrap()),
            Value::U16(v) => Value::U16(v * rhs.as_u16().unwrap()),
            Value::U32(v) => Value::U32(v * rhs.as_u32().unwrap()),
            Value::U64(v) => Value::U64(v * rhs.as_u64().unwrap()),
            Value::U128(v) => Value::U128(v * rhs.as_u128().unwrap()),
            Value::F32(v) => Value::F32(v * rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v * rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d * rhs.as_decimal().unwrap()),
            _ => Value::Null,
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::I8(v) => Value::I8(v / rhs.as_i8().unwrap()),
            Value::I16(v) => Value::I16(v / rhs.as_i16().unwrap()),
            Value::I32(v) => Value::I32(v / rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v / rhs.as_i64().unwrap()),
            Value::I128(v) => Value::I128(v / rhs.as_i128().unwrap()),
            Value::U8(v) => Value::U8(v / rhs.as_u8().unwrap()),
            Value::U16(v) => Value::U16(v / rhs.as_u16().unwrap()),
            Value::U32(v) => Value::U32(v / rhs.as_u32().unwrap()),
            Value::U64(v) => Value::U64(v / rhs.as_u64().unwrap()),
            Value::U128(v) => Value::U128(v / rhs.as_u128().unwrap()),
            Value::F32(v) => Value::F32(v / rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v / rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d / rhs.as_decimal().unwrap()),
            _ => Value::Null,
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Value::I8(v) => Value::I8(v % rhs.as_i8().unwrap()),
            Value::I16(v) => Value::I16(v % rhs.as_i16().unwrap()),
            Value::I32(v) => Value::I32(v % rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v % rhs.as_i64().unwrap()),
            Value::I128(v) => Value::I128(v % rhs.as_i128().unwrap()),
            Value::U8(v) => Value::U8(v % rhs.as_u8().unwrap()),
            Value::U16(v) => Value::U16(v % rhs.as_u16().unwrap()),
            Value::U32(v) => Value::U32(v % rhs.as_u32().unwrap()),
            Value::U64(v) => Value::U64(v % rhs.as_u64().unwrap()),
            Value::U128(v) => Value::U128(v % rhs.as_u128().unwrap()),
            Value::F32(v) => Value::F32(v % rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v % rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d % rhs.as_decimal().unwrap()),
            _ => Value::Null,
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Value {
        match self {
            Value::Bool(val) => {
                Value::Bool(if val { false } else { true })
            }
            Value::I8(val) => {
                Value::I8(-val)
            }
            Value::I16(val) => {
                Value::I16(-val)
            }
            Value::I32(val) => {
                Value::I32(-val)
            }
            Value::I64(val) => {
                Value::I64(-val)
            }
            Value::I128(val) => {
                Value::I128(-val)
            }
            Value::F32(val) => {
                Value::F32(-val)
            }
            Value::F64(val) => {
                Value::F64(-val)
            }
            Value::Decimal(val) => {
                Value::Decimal(-val)
            }
            Value::U8(val) => {
                Value::I8(-(val as i8))
            }
            Value::U16(val) => {
                Value::I16(-(val as i16))
            }
            Value::U32(val) => {
                Value::I32(-(val as i32))
            }
            Value::U64(val) => {
                Value::I64(-(val as i64))
            }
            Value::U128(val) => {
                Value::I128(-(val as i128))
            }
            _ => Value::Null,
        }
    }
}

impl Neg for &Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        (*self).neg()
    }
}
