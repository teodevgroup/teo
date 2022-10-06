use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Add, Div, Mul, Sub, Rem};
use async_recursion::async_recursion;
use chrono::prelude::{Date, DateTime, Utc};
use chrono::SecondsFormat;
use rust_decimal::Decimal;
#[cfg(feature = "data-source-mongodb")]
use bson::oid::ObjectId;
use crate::core::field::r#type::FieldType;
use crate::core::object::Object;

pub mod index;

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
    BTreeSet(HashSet<Value>),

    /// Represents a Tson object.
    ///
    Object(Object),
}
