pub mod index;
pub mod json;
pub mod from;
pub mod macros;
pub mod range;
pub(crate) mod decoder;
pub(crate) mod utils;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::mem;
use std::ops::{Add, Div, Mul, Sub, Rem, Neg, BitAnd, BitXor, BitOr};
use chrono::prelude::{DateTime, Utc};
use indexmap::IndexMap;
#[cfg(feature = "data-source-mongodb")]
use bson::oid::ObjectId;
use chrono::NaiveDate;
use maplit::hashmap;
use regex::Regex;
use bigdecimal::BigDecimal;
use crate::core::field::r#type::FieldType;
use crate::core::object::Object;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::pipeline::Pipeline;
use crate::core::teon::index::Index;
use crate::core::teon::range::Range;
use crate::core::result::Result;
use crate::prelude::Error;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/mod.rs

/// Represents any valid Tson value. A Tson value is an extension for Teo just like Bson for
/// MongoDB.
///
#[derive(Debug, Clone)]
pub enum Value {

    /// Represents a JSON null value.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(null);
    /// ```
    Null,

    /// Represents a Tson bool.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(true);
    /// ```
    Bool(bool),

    /// Represents a Tson i32.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(12_i32);
    /// ```
    I32(i32),

    /// Represents a Tson i64.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(12_i64);
    /// ```
    I64(i64),

    /// Represents a Tson f32.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(12.5_f32);
    /// ```
    F32(f32),

    /// Represents a Tson f64.
    ///
    /// ```
    /// # use teo::prelude::teon;
    /// #
    /// let v = teon!(12.5_f64);
    /// ```
    F64(f64),

    /// Represents a Tson decimal.
    ///
    Decimal(BigDecimal),

    /// Represents a Tson object id.
    ///
    #[cfg(feature = "data-source-mongodb")]
    ObjectId(ObjectId),

    /// Represents a Tson string.
    ///
    String(String),

    /// Represents a Tson date.
    ///
    Date(NaiveDate),

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

    /// Represents a Tson btreemap.
    ///
    IndexMap(IndexMap<String, Value>),

    /// Represents a Tson range.
    ///
    Range(Range),

    /// Represents a Tson tuple.
    ///
    Tuple(Vec<Value>),

    /// Represents a Tson pipeline.
    ///
    Pipeline(Pipeline),

    /// Raw enum choice.
    ///
    RawEnumChoice(String, Option<Vec<(Option<String>, Value)>>),

    /// Raw option choice
    ///
    RawOptionChoice(u32),

    /// Regular expression
    ///
    RegExp(Regex),

    /// Represents a Tson object.
    ///
    Object(Object),
}

impl Value {

    pub(crate) fn number_from_i32(num: i32, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::I32 => Value::I32(num as i32),
            _ => panic!(),
        }
    }

    pub(crate) fn number_from_f64(num: f64, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::F32 => Value::F32(num as f32),
            FieldType::F64 => Value::F64(num),
            _ => panic!()
        }
    }
    pub(crate) fn number_from_f32(num: f32, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::F32 => Value::F32(num),
            FieldType::F64 => Value::F64(num as f64),
            _ => panic!()
        }
    }

    pub(crate) fn number_from_i64(num: i64, r#type: &FieldType) -> Value {
        match r#type {
            FieldType::I32 => Value::I32(num as i32),
            FieldType::I64 => Value::I64(num),
            _ => panic!()
        }
    }

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

    pub fn is_indexmap(&self) -> bool {
        self.as_indexmap().is_some()
    }

    pub fn as_indexmap(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::IndexMap(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_indexmap_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::IndexMap(map) => Some(map),
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

    pub fn str_from_string_or_raw_enum_choice(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::RawEnumChoice(s, _) => Some(s),
            _ => None,
        }
    }

    pub fn is_i(&self) -> bool {
        match *self {
            Value::I32(_) | Value::I64(_) => true,
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
        self.is_i() || self.is_f()
    }

    pub fn is_i32(&self) -> bool {
        match *self {
            Value::I32(_) => true,
            _ => false,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Value::I32(v) => Some(v),
            Value::I64(v) => Some(v as i32),
            Value::F32(f) => Some(f as i32),
            Value::F64(f) => Some(f as i32),
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
            Value::I32(v) => Some(v as i64),
            Value::I64(v) => Some(v),
            Value::F32(f) => Some(f as i64),
            Value::F64(f) => Some(f as i64),
            _ => None
        }
    }

    pub fn is_f32(&self) -> bool {
        match *self {
            Value::F32(_v) => true,
            _ => false,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Value::I32(v) => Some(v as f32),
            Value::I64(v) => Some(v as f32),
            Value::F32(v) => Some(v),
            Value::F64(v) => Some(v as f32),
            _ => None
        }
    }

    pub fn is_f64(&self) -> bool {
        match *self {
            Value::F64(_v) => true,
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::I32(v) => Some(v as f64),
            Value::I64(v) => Some(v as f64),
            Value::F32(v) => Some(v as f64),
            Value::F64(v) => Some(v as f64),
            _ => None
        }
    }

    pub fn is_decimal(&self) -> bool {
        match *self {
            Value::Decimal(_) => true,
            _ => false,
        }
    }

    pub fn as_decimal(&self) -> Option<BigDecimal> {
        match self {
            Value::Decimal(v) => Some(v.clone()),
            _ => None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            Value::I32(n) => Some(*n as usize),
            Value::I64(n) => Some(*n as usize),
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

    pub fn as_date(&self) -> Option<&NaiveDate> {
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

    #[cfg(feature = "data-source-mongodb")]
    pub fn is_object_id(&self) -> bool {
        self.as_object_id().is_some()
    }

    #[cfg(feature = "data-source-mongodb")]
    pub fn as_object_id(&self) -> Option<&ObjectId> {
        match self {
            Value::ObjectId(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(o) => Some(o),
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

    pub fn is_raw_enum_choice(&self) -> bool {
        self.as_raw_enum_choice().is_some()
    }

    pub fn as_raw_enum_choice(&self) -> Option<&str> {
        match self {
            Value::RawEnumChoice(s, _) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn is_raw_option_choice(&self) -> bool {
        self.as_raw_option_choice().is_some()
    }

    pub fn as_raw_option_choice(&self) -> Option<u32> {
        match self {
            Value::RawOptionChoice(o) => Some(*o),
            _ => None,
        }
    }

    pub fn is_range(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_range(&self) -> Option<&Range> {
        match self {
            Value::Range(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_tuple(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_tuple(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Tuple(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_pipeline(&self) -> bool {
        self.as_range().is_some()
    }

    pub fn as_pipeline(&self) -> Option<&Pipeline> {
        match self {
            Value::Pipeline(p) => Some(p),
            _ => None,
        }
    }

    pub fn is_regexp(&self) -> bool {
        self.as_regexp().is_some()
    }

    pub fn as_regexp(&self) -> Option<&Regex> {
        match self {
            Value::RegExp(r) => Some(r),
            _ => None,
        }
    }

    // resolve pipeline as value
    pub(crate) async fn resolve(&self, context: PipelineCtx<'_>) -> Result<Value> {
        match self {
            Value::Pipeline(p) => p.process(context).await,
            Value::HashMap(map) => {
                let mut new_map = hashmap!{};
                for (key, value) in map {
                    if let Some(p) = value.as_pipeline() {
                        new_map.insert(key.clone(), p.process(context.clone()).await?);
                    } else {
                        new_map.insert(key.clone(), value.clone());
                    }
                }
                Ok(Value::HashMap(new_map))
            }
            Value::Vec(vec) => {
                let mut new_vec = vec![];
                for val in vec {
                    if let Some(p) = val.as_pipeline() {
                        new_vec.push(p.process(context.clone()).await?);
                    } else {
                        new_vec.push(val.clone());
                    }
                }
                Ok(Value::Vec(new_vec))
            }
            _ => Ok(self.clone()),
        }
    }

    /// Takes the value out of the `Value`, leaving a `Null` in its place.
    ///
    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    pub(crate) fn recip(&self) -> f64 {
        match self {
            Value::I32(n) => (*n as f64).recip(),
            Value::I64(n) => (*n as f64).recip(),
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
            #[cfg(feature = "data-source-mongodb")]
            (ObjectId(s), ObjectId(o)) => s.partial_cmp(o),
            (Bool(s), Bool(o)) => s.partial_cmp(o),
            (I32(s), I32(o)) => s.partial_cmp(o),
            (I64(s), I64(o)) => s.partial_cmp(o),
            (F32(s), F32(o)) => s.partial_cmp(o),
            (F64(s), F64(o)) => s.partial_cmp(o),
            (Decimal(s), Decimal(o)) => s.partial_cmp(o),
            (String(s), String(o)) => s.partial_cmp(o),
            (Date(s), Date(o)) => s.partial_cmp(o),
            (DateTime(s), DateTime(o)) => s.partial_cmp(o),
            (Vec(s), Vec(o)) => s.partial_cmp(o),
            (HashMap(_s), HashMap(_o)) => None,
            (BTreeMap(_s), BTreeMap(_o)) => None,
            (Object(_s), Object(_o)) => None,
            _ => None,
        }
    }
}

fn check_operand(lhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_number() {
        return Err(Error::internal_server_error(format!("{}: operand is not number", name)));
    }
    Ok(())
}

fn check_operands(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_number() {
        return Err(Error::internal_server_error(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_number() {
        return Err(Error::internal_server_error(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

fn check_operands_int(lhs: &Value, rhs: &Value, name: &str) -> Result<()> {
    if !lhs.is_i() {
        return Err(Error::internal_server_error(format!("{}: lhs is not number", name)));
    }
    if !rhs.is_i() {
        return Err(Error::internal_server_error(format!("{}: rhs is not number", name)));
    }
    Ok(())
}

impl Add for Value {
    type Output = Result<Value>;
    fn add(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "add")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v + rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v + rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v + rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v + rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d + rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Sub for Value {
    type Output = Result<Value>;
    fn sub(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "sub")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v - rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v - rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v - rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v - rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d - rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Mul for Value {
    type Output = Result<Value>;
    fn mul(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "mul")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v * rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v * rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v * rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v * rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d * rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Div for Value {
    type Output = Result<Value>;
    fn div(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "div")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v / rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v / rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v / rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v / rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d / rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Rem for Value {
    type Output = Result<Value>;
    fn rem(self, rhs: Self) -> Self::Output {
        check_operands(&self, &rhs, "rem")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v % rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v % rhs.as_i64().unwrap()),
            Value::F32(v) => Value::F32(v % rhs.as_f32().unwrap()),
            Value::F64(v) => Value::F64(v % rhs.as_f64().unwrap()),
            Value::Decimal(d) => Value::Decimal(d % rhs.as_decimal().unwrap()),
            _ => unreachable!(),
        })
    }
}

impl Neg for Value {
    type Output = Result<Value>;
    fn neg(self) -> Self::Output {
        check_operand(&self, "neg")?;
        Ok(match self {
            Value::I32(val) => Value::I32(-val),
            Value::I64(val) => Value::I64(-val),
            Value::F32(val) => Value::F32(-val),
            Value::F64(val) => Value::F64(-val),
            Value::Decimal(val) => Value::Decimal(-val),
            _ => unreachable!(),
        })
    }
}

impl BitAnd for Value {
    type Output = Result<Value>;
    fn bitand(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitand")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v & rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v & rhs.as_i64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitXor for Value {
    type Output = Result<Value>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitxor")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v ^ rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v ^ rhs.as_i64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl BitOr for Value {
    type Output = Result<Value>;
    fn bitor(self, rhs: Self) -> Self::Output {
        check_operands_int(&self, &rhs, "bitor")?;
        Ok(match self {
            Value::I32(v) => Value::I32(v | rhs.as_i32().unwrap()),
            Value::I64(v) => Value::I64(v | rhs.as_i64().unwrap()),
            _ => Value::Null,
        })
    }
}

impl Neg for &Value {
    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        (self.clone()).neg()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        if self.is_i() && other.is_i() {
            return self.as_i64().unwrap() == other.as_i64().unwrap();
        }
        if self.is_number() && other.is_number() {
            return self.as_f64().unwrap() == other.as_f64().unwrap();
        }
        match (self, other) {
            (Null, Null) => true,
            #[cfg(feature = "data-source-mongodb")]
            (ObjectId(s), ObjectId(o)) => s == o,
            (Bool(s), Bool(o)) => s == o,
            (I32(s), I32(o)) => s == o,
            (I64(s), I64(o)) => s == o,
            (F32(s), F32(o)) => s == o,
            (F64(s), F64(o)) => s == o,
            (Decimal(s), Decimal(o)) => s == o,
            (String(s), String(o)) => s == o,
            (Date(s), Date(o)) => s == o,
            (DateTime(s), DateTime(o)) => s == o,
            (Vec(s), Vec(o)) => s == o,
            (HashMap(s), HashMap(o)) => s == o,
            (IndexMap(s), IndexMap(o)) => s == o,
            (BTreeMap(s), BTreeMap(o)) => s == o,
            (RawEnumChoice(s1, _a1), RawEnumChoice(s2, a2)) => s1 == s2 && a2 == a2,
            _ => false,
        }
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        &self
    }
}
