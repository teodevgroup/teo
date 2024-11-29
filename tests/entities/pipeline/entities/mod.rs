#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod helpers;
pub mod stdlib;

use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};


/// ## Status
///
/// This enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct Status {
    inner: String,
}

impl Status {
    /// ### Is Open
    ///
    /// Returns true if value is open
    pub fn is_open(&self) -> bool {
        self.inner.as_str() == "open"
    }
    /// ### Open
    ///
    /// This enum member doesn't have a description.
    pub fn open() -> Self {
        Self { inner: "open".to_owned() }
    }
    /// ### Is In progress
    ///
    /// Returns true if value is in progress
    pub fn is_in_progress(&self) -> bool {
        self.inner.as_str() == "inProgress"
    }
    /// ### In progress
    ///
    /// This enum member doesn't have a description.
    pub fn in_progress() -> Self {
        Self { inner: "inProgress".to_owned() }
    }
    /// ### Is Pending
    ///
    /// Returns true if value is pending
    pub fn is_pending(&self) -> bool {
        self.inner.as_str() == "pending"
    }
    /// ### Pending
    ///
    /// This enum member doesn't have a description.
    pub fn pending() -> Self {
        Self { inner: "pending".to_owned() }
    }
    /// ### Is Waiting for review
    ///
    /// Returns true if value is waiting for review
    pub fn is_waiting_for_review(&self) -> bool {
        self.inner.as_str() == "waitingForReview"
    }
    /// ### Waiting for review
    ///
    /// This enum member doesn't have a description.
    pub fn waiting_for_review() -> Self {
        Self { inner: "waitingForReview".to_owned() }
    }
    /// ### Is Done
    ///
    /// Returns true if value is done
    pub fn is_done(&self) -> bool {
        self.inner.as_str() == "done"
    }
    /// ### Done
    ///
    /// This enum member doesn't have a description.
    pub fn done() -> Self {
        Self { inner: "done".to_owned() }
    }
}

impl From<Status> for Value {
    fn from(value: Status) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for Status {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "open" => Status::open(),
                "inProgress" => Status::in_progress(),
                "pending" => Status::pending(),
                "waitingForReview" => Status::waiting_for_review(),
                "done" => Status::done(),
                _ => Err(Error::new("cannot convert value to Status"))?
            })
        } else {
            Err(Error::new("cannot convert value to Status"))
        }
    }
}

impl TryFrom<&Value> for Status {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "open" => Status::open(),
                "inProgress" => Status::in_progress(),
                "pending" => Status::pending(),
                "waitingForReview" => Status::waiting_for_review(),
                "done" => Status::done(),
                _ => Err(Error::new("cannot convert value to Status"))?
            })
        } else {
            Err(Error::new("cannot convert value to Status"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &Status {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "open" => unsafe { &*(enum_variant as *const str as *const Self) },
                "inProgress" => unsafe { &*(enum_variant as *const str as *const Self) },
                "pending" => unsafe { &*(enum_variant as *const str as *const Self) },
                "waitingForReview" => unsafe { &*(enum_variant as *const str as *const Self) },
                "done" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Status"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Status"))
        }
    }
}

impl AsInterface for Status {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "open" => Status::open(),
                "inProgress" => Status::in_progress(),
                "pending" => Status::pending(),
                "waitingForReview" => Status::waiting_for_review(),
                "done" => Status::done(),
                _ => Err(Error::new("cannot convert value to Status"))?
            })
        } else {
            Err(Error::new("cannot convert value to Status"))
        }
    }
}

impl AsInterfaceRef for Status {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "open" => unsafe { &*(enum_variant as *const str as *const Self) },
                "inProgress" => unsafe { &*(enum_variant as *const str as *const Self) },
                "pending" => unsafe { &*(enum_variant as *const str as *const Self) },
                "waitingForReview" => unsafe { &*(enum_variant as *const str as *const Self) },
                "done" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Status"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Status"))
        }
    }
}
/// ## Container scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct ContainerScalarFields {
    inner: String,
}

impl ContainerScalarFields {
    /// ### Is Bool
    ///
    /// Returns true if value is bool
    pub fn is_bool(&self) -> bool {
        self.inner.as_str() == "bool"
    }
    /// ### Bool
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn bool() -> Self {
        Self { inner: "bool".to_owned() }
    }
    /// ### Is Bool array
    ///
    /// Returns true if value is bool array
    pub fn is_bool_array(&self) -> bool {
        self.inner.as_str() == "boolArray"
    }
    /// ### Bool array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn bool_array() -> Self {
        Self { inner: "boolArray".to_owned() }
    }
    /// ### Is Date
    ///
    /// Returns true if value is date
    pub fn is_date(&self) -> bool {
        self.inner.as_str() == "date"
    }
    /// ### Date
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date() -> Self {
        Self { inner: "date".to_owned() }
    }
    /// ### Is Date array
    ///
    /// Returns true if value is date array
    pub fn is_date_array(&self) -> bool {
        self.inner.as_str() == "dateArray"
    }
    /// ### Date array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_array() -> Self {
        Self { inner: "dateArray".to_owned() }
    }
    /// ### Is Date time
    ///
    /// Returns true if value is date time
    pub fn is_date_time(&self) -> bool {
        self.inner.as_str() == "dateTime"
    }
    /// ### Date time
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_time() -> Self {
        Self { inner: "dateTime".to_owned() }
    }
    /// ### Is Date time array
    ///
    /// Returns true if value is date time array
    pub fn is_date_time_array(&self) -> bool {
        self.inner.as_str() == "dateTimeArray"
    }
    /// ### Date time array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_time_array() -> Self {
        Self { inner: "dateTimeArray".to_owned() }
    }
    /// ### Is Decimal
    ///
    /// Returns true if value is decimal
    pub fn is_decimal(&self) -> bool {
        self.inner.as_str() == "decimal"
    }
    /// ### Decimal
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn decimal() -> Self {
        Self { inner: "decimal".to_owned() }
    }
    /// ### Is Decimal array
    ///
    /// Returns true if value is decimal array
    pub fn is_decimal_array(&self) -> bool {
        self.inner.as_str() == "decimalArray"
    }
    /// ### Decimal array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn decimal_array() -> Self {
        Self { inner: "decimalArray".to_owned() }
    }
    /// ### Is Float32
    ///
    /// Returns true if value is float32
    pub fn is_float_32(&self) -> bool {
        self.inner.as_str() == "float32"
    }
    /// ### Float32
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_32() -> Self {
        Self { inner: "float32".to_owned() }
    }
    /// ### Is Float32 array
    ///
    /// Returns true if value is float32 array
    pub fn is_float_32_array(&self) -> bool {
        self.inner.as_str() == "float32Array"
    }
    /// ### Float32 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_32_array() -> Self {
        Self { inner: "float32Array".to_owned() }
    }
    /// ### Is Float64
    ///
    /// Returns true if value is float64
    pub fn is_float_64(&self) -> bool {
        self.inner.as_str() == "float64"
    }
    /// ### Float64
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_64() -> Self {
        Self { inner: "float64".to_owned() }
    }
    /// ### Is Float64 array
    ///
    /// Returns true if value is float64 array
    pub fn is_float_64_array(&self) -> bool {
        self.inner.as_str() == "float64Array"
    }
    /// ### Float64 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_64_array() -> Self {
        Self { inner: "float64Array".to_owned() }
    }
    /// ### Is Id
    ///
    /// Returns true if value is id
    pub fn is_id(&self) -> bool {
        self.inner.as_str() == "id"
    }
    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn id() -> Self {
        Self { inner: "id".to_owned() }
    }
    /// ### Is Int32
    ///
    /// Returns true if value is int32
    pub fn is_int_32(&self) -> bool {
        self.inner.as_str() == "int32"
    }
    /// ### Int32
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_32() -> Self {
        Self { inner: "int32".to_owned() }
    }
    /// ### Is Int32 array
    ///
    /// Returns true if value is int32 array
    pub fn is_int_32_array(&self) -> bool {
        self.inner.as_str() == "int32Array"
    }
    /// ### Int32 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_32_array() -> Self {
        Self { inner: "int32Array".to_owned() }
    }
    /// ### Is Int64
    ///
    /// Returns true if value is int64
    pub fn is_int_64(&self) -> bool {
        self.inner.as_str() == "int64"
    }
    /// ### Int64
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_64() -> Self {
        Self { inner: "int64".to_owned() }
    }
    /// ### Is Int64 array
    ///
    /// Returns true if value is int64 array
    pub fn is_int_64_array(&self) -> bool {
        self.inner.as_str() == "int64Array"
    }
    /// ### Int64 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_64_array() -> Self {
        Self { inner: "int64Array".to_owned() }
    }
    /// ### Is Message
    ///
    /// Returns true if value is message
    pub fn is_message(&self) -> bool {
        self.inner.as_str() == "message"
    }
    /// ### Message
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn message() -> Self {
        Self { inner: "message".to_owned() }
    }
    /// ### Is Status
    ///
    /// Returns true if value is status
    pub fn is_status(&self) -> bool {
        self.inner.as_str() == "status"
    }
    /// ### Status
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn status() -> Self {
        Self { inner: "status".to_owned() }
    }
    /// ### Is Status array
    ///
    /// Returns true if value is status array
    pub fn is_status_array(&self) -> bool {
        self.inner.as_str() == "statusArray"
    }
    /// ### Status array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn status_array() -> Self {
        Self { inner: "statusArray".to_owned() }
    }
    /// ### Is String
    ///
    /// Returns true if value is string
    pub fn is_string(&self) -> bool {
        self.inner.as_str() == "string"
    }
    /// ### String
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn string() -> Self {
        Self { inner: "string".to_owned() }
    }
    /// ### Is String array
    ///
    /// Returns true if value is string array
    pub fn is_string_array(&self) -> bool {
        self.inner.as_str() == "stringArray"
    }
    /// ### String array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn string_array() -> Self {
        Self { inner: "stringArray".to_owned() }
    }
}

impl From<ContainerScalarFields> for Value {
    fn from(value: ContainerScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for ContainerScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => ContainerScalarFields::bool(),
                "boolArray" => ContainerScalarFields::bool_array(),
                "date" => ContainerScalarFields::date(),
                "dateArray" => ContainerScalarFields::date_array(),
                "dateTime" => ContainerScalarFields::date_time(),
                "dateTimeArray" => ContainerScalarFields::date_time_array(),
                "decimal" => ContainerScalarFields::decimal(),
                "decimalArray" => ContainerScalarFields::decimal_array(),
                "float32" => ContainerScalarFields::float_32(),
                "float32Array" => ContainerScalarFields::float_32_array(),
                "float64" => ContainerScalarFields::float_64(),
                "float64Array" => ContainerScalarFields::float_64_array(),
                "id" => ContainerScalarFields::id(),
                "int32" => ContainerScalarFields::int_32(),
                "int32Array" => ContainerScalarFields::int_32_array(),
                "int64" => ContainerScalarFields::int_64(),
                "int64Array" => ContainerScalarFields::int_64_array(),
                "message" => ContainerScalarFields::message(),
                "status" => ContainerScalarFields::status(),
                "statusArray" => ContainerScalarFields::status_array(),
                "string" => ContainerScalarFields::string(),
                "stringArray" => ContainerScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to ContainerScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &ContainerScalarFields {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => unsafe { &*(enum_variant as *const str as *const Self) },
                "boolArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "date" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTime" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTimeArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimal" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimalArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "message" => unsafe { &*(enum_variant as *const str as *const Self) },
                "status" => unsafe { &*(enum_variant as *const str as *const Self) },
                "statusArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &ContainerScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerScalarFields"))
        }
    }
}

impl AsInterface for ContainerScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => ContainerScalarFields::bool(),
                "boolArray" => ContainerScalarFields::bool_array(),
                "date" => ContainerScalarFields::date(),
                "dateArray" => ContainerScalarFields::date_array(),
                "dateTime" => ContainerScalarFields::date_time(),
                "dateTimeArray" => ContainerScalarFields::date_time_array(),
                "decimal" => ContainerScalarFields::decimal(),
                "decimalArray" => ContainerScalarFields::decimal_array(),
                "float32" => ContainerScalarFields::float_32(),
                "float32Array" => ContainerScalarFields::float_32_array(),
                "float64" => ContainerScalarFields::float_64(),
                "float64Array" => ContainerScalarFields::float_64_array(),
                "id" => ContainerScalarFields::id(),
                "int32" => ContainerScalarFields::int_32(),
                "int32Array" => ContainerScalarFields::int_32_array(),
                "int64" => ContainerScalarFields::int_64(),
                "int64Array" => ContainerScalarFields::int_64_array(),
                "message" => ContainerScalarFields::message(),
                "status" => ContainerScalarFields::status(),
                "statusArray" => ContainerScalarFields::status_array(),
                "string" => ContainerScalarFields::string(),
                "stringArray" => ContainerScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to ContainerScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerScalarFields"))
        }
    }
}

impl AsInterfaceRef for ContainerScalarFields {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => unsafe { &*(enum_variant as *const str as *const Self) },
                "boolArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "date" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTime" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTimeArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimal" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimalArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "message" => unsafe { &*(enum_variant as *const str as *const Self) },
                "status" => unsafe { &*(enum_variant as *const str as *const Self) },
                "statusArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &ContainerScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerScalarFields"))
        }
    }
}
/// ## Container serializable scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct ContainerSerializableScalarFields {
    inner: String,
}

impl ContainerSerializableScalarFields {
    /// ### Is Bool
    ///
    /// Returns true if value is bool
    pub fn is_bool(&self) -> bool {
        self.inner.as_str() == "bool"
    }
    /// ### Bool
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn bool() -> Self {
        Self { inner: "bool".to_owned() }
    }
    /// ### Is Bool array
    ///
    /// Returns true if value is bool array
    pub fn is_bool_array(&self) -> bool {
        self.inner.as_str() == "boolArray"
    }
    /// ### Bool array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn bool_array() -> Self {
        Self { inner: "boolArray".to_owned() }
    }
    /// ### Is Date
    ///
    /// Returns true if value is date
    pub fn is_date(&self) -> bool {
        self.inner.as_str() == "date"
    }
    /// ### Date
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date() -> Self {
        Self { inner: "date".to_owned() }
    }
    /// ### Is Date array
    ///
    /// Returns true if value is date array
    pub fn is_date_array(&self) -> bool {
        self.inner.as_str() == "dateArray"
    }
    /// ### Date array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_array() -> Self {
        Self { inner: "dateArray".to_owned() }
    }
    /// ### Is Date time
    ///
    /// Returns true if value is date time
    pub fn is_date_time(&self) -> bool {
        self.inner.as_str() == "dateTime"
    }
    /// ### Date time
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_time() -> Self {
        Self { inner: "dateTime".to_owned() }
    }
    /// ### Is Date time array
    ///
    /// Returns true if value is date time array
    pub fn is_date_time_array(&self) -> bool {
        self.inner.as_str() == "dateTimeArray"
    }
    /// ### Date time array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn date_time_array() -> Self {
        Self { inner: "dateTimeArray".to_owned() }
    }
    /// ### Is Decimal
    ///
    /// Returns true if value is decimal
    pub fn is_decimal(&self) -> bool {
        self.inner.as_str() == "decimal"
    }
    /// ### Decimal
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn decimal() -> Self {
        Self { inner: "decimal".to_owned() }
    }
    /// ### Is Decimal array
    ///
    /// Returns true if value is decimal array
    pub fn is_decimal_array(&self) -> bool {
        self.inner.as_str() == "decimalArray"
    }
    /// ### Decimal array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn decimal_array() -> Self {
        Self { inner: "decimalArray".to_owned() }
    }
    /// ### Is Float32
    ///
    /// Returns true if value is float32
    pub fn is_float_32(&self) -> bool {
        self.inner.as_str() == "float32"
    }
    /// ### Float32
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_32() -> Self {
        Self { inner: "float32".to_owned() }
    }
    /// ### Is Float32 array
    ///
    /// Returns true if value is float32 array
    pub fn is_float_32_array(&self) -> bool {
        self.inner.as_str() == "float32Array"
    }
    /// ### Float32 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_32_array() -> Self {
        Self { inner: "float32Array".to_owned() }
    }
    /// ### Is Float64
    ///
    /// Returns true if value is float64
    pub fn is_float_64(&self) -> bool {
        self.inner.as_str() == "float64"
    }
    /// ### Float64
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_64() -> Self {
        Self { inner: "float64".to_owned() }
    }
    /// ### Is Float64 array
    ///
    /// Returns true if value is float64 array
    pub fn is_float_64_array(&self) -> bool {
        self.inner.as_str() == "float64Array"
    }
    /// ### Float64 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn float_64_array() -> Self {
        Self { inner: "float64Array".to_owned() }
    }
    /// ### Is Id
    ///
    /// Returns true if value is id
    pub fn is_id(&self) -> bool {
        self.inner.as_str() == "id"
    }
    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn id() -> Self {
        Self { inner: "id".to_owned() }
    }
    /// ### Is Int32
    ///
    /// Returns true if value is int32
    pub fn is_int_32(&self) -> bool {
        self.inner.as_str() == "int32"
    }
    /// ### Int32
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_32() -> Self {
        Self { inner: "int32".to_owned() }
    }
    /// ### Is Int32 array
    ///
    /// Returns true if value is int32 array
    pub fn is_int_32_array(&self) -> bool {
        self.inner.as_str() == "int32Array"
    }
    /// ### Int32 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_32_array() -> Self {
        Self { inner: "int32Array".to_owned() }
    }
    /// ### Is Int64
    ///
    /// Returns true if value is int64
    pub fn is_int_64(&self) -> bool {
        self.inner.as_str() == "int64"
    }
    /// ### Int64
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_64() -> Self {
        Self { inner: "int64".to_owned() }
    }
    /// ### Is Int64 array
    ///
    /// Returns true if value is int64 array
    pub fn is_int_64_array(&self) -> bool {
        self.inner.as_str() == "int64Array"
    }
    /// ### Int64 array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn int_64_array() -> Self {
        Self { inner: "int64Array".to_owned() }
    }
    /// ### Is Message
    ///
    /// Returns true if value is message
    pub fn is_message(&self) -> bool {
        self.inner.as_str() == "message"
    }
    /// ### Message
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn message() -> Self {
        Self { inner: "message".to_owned() }
    }
    /// ### Is Status
    ///
    /// Returns true if value is status
    pub fn is_status(&self) -> bool {
        self.inner.as_str() == "status"
    }
    /// ### Status
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn status() -> Self {
        Self { inner: "status".to_owned() }
    }
    /// ### Is Status array
    ///
    /// Returns true if value is status array
    pub fn is_status_array(&self) -> bool {
        self.inner.as_str() == "statusArray"
    }
    /// ### Status array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn status_array() -> Self {
        Self { inner: "statusArray".to_owned() }
    }
    /// ### Is String
    ///
    /// Returns true if value is string
    pub fn is_string(&self) -> bool {
        self.inner.as_str() == "string"
    }
    /// ### String
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn string() -> Self {
        Self { inner: "string".to_owned() }
    }
    /// ### Is String array
    ///
    /// Returns true if value is string array
    pub fn is_string_array(&self) -> bool {
        self.inner.as_str() == "stringArray"
    }
    /// ### String array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn string_array() -> Self {
        Self { inner: "stringArray".to_owned() }
    }
}

impl From<ContainerSerializableScalarFields> for Value {
    fn from(value: ContainerSerializableScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for ContainerSerializableScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => ContainerSerializableScalarFields::bool(),
                "boolArray" => ContainerSerializableScalarFields::bool_array(),
                "date" => ContainerSerializableScalarFields::date(),
                "dateArray" => ContainerSerializableScalarFields::date_array(),
                "dateTime" => ContainerSerializableScalarFields::date_time(),
                "dateTimeArray" => ContainerSerializableScalarFields::date_time_array(),
                "decimal" => ContainerSerializableScalarFields::decimal(),
                "decimalArray" => ContainerSerializableScalarFields::decimal_array(),
                "float32" => ContainerSerializableScalarFields::float_32(),
                "float32Array" => ContainerSerializableScalarFields::float_32_array(),
                "float64" => ContainerSerializableScalarFields::float_64(),
                "float64Array" => ContainerSerializableScalarFields::float_64_array(),
                "id" => ContainerSerializableScalarFields::id(),
                "int32" => ContainerSerializableScalarFields::int_32(),
                "int32Array" => ContainerSerializableScalarFields::int_32_array(),
                "int64" => ContainerSerializableScalarFields::int_64(),
                "int64Array" => ContainerSerializableScalarFields::int_64_array(),
                "message" => ContainerSerializableScalarFields::message(),
                "status" => ContainerSerializableScalarFields::status(),
                "statusArray" => ContainerSerializableScalarFields::status_array(),
                "string" => ContainerSerializableScalarFields::string(),
                "stringArray" => ContainerSerializableScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to ContainerSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerSerializableScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &ContainerSerializableScalarFields {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => unsafe { &*(enum_variant as *const str as *const Self) },
                "boolArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "date" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTime" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTimeArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimal" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimalArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "message" => unsafe { &*(enum_variant as *const str as *const Self) },
                "status" => unsafe { &*(enum_variant as *const str as *const Self) },
                "statusArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &ContainerSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerSerializableScalarFields"))
        }
    }
}

impl AsInterface for ContainerSerializableScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => ContainerSerializableScalarFields::bool(),
                "boolArray" => ContainerSerializableScalarFields::bool_array(),
                "date" => ContainerSerializableScalarFields::date(),
                "dateArray" => ContainerSerializableScalarFields::date_array(),
                "dateTime" => ContainerSerializableScalarFields::date_time(),
                "dateTimeArray" => ContainerSerializableScalarFields::date_time_array(),
                "decimal" => ContainerSerializableScalarFields::decimal(),
                "decimalArray" => ContainerSerializableScalarFields::decimal_array(),
                "float32" => ContainerSerializableScalarFields::float_32(),
                "float32Array" => ContainerSerializableScalarFields::float_32_array(),
                "float64" => ContainerSerializableScalarFields::float_64(),
                "float64Array" => ContainerSerializableScalarFields::float_64_array(),
                "id" => ContainerSerializableScalarFields::id(),
                "int32" => ContainerSerializableScalarFields::int_32(),
                "int32Array" => ContainerSerializableScalarFields::int_32_array(),
                "int64" => ContainerSerializableScalarFields::int_64(),
                "int64Array" => ContainerSerializableScalarFields::int_64_array(),
                "message" => ContainerSerializableScalarFields::message(),
                "status" => ContainerSerializableScalarFields::status(),
                "statusArray" => ContainerSerializableScalarFields::status_array(),
                "string" => ContainerSerializableScalarFields::string(),
                "stringArray" => ContainerSerializableScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to ContainerSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerSerializableScalarFields"))
        }
    }
}

impl AsInterfaceRef for ContainerSerializableScalarFields {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => unsafe { &*(enum_variant as *const str as *const Self) },
                "boolArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "date" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTime" => unsafe { &*(enum_variant as *const str as *const Self) },
                "dateTimeArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimal" => unsafe { &*(enum_variant as *const str as *const Self) },
                "decimalArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "float64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int32Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64" => unsafe { &*(enum_variant as *const str as *const Self) },
                "int64Array" => unsafe { &*(enum_variant as *const str as *const Self) },
                "message" => unsafe { &*(enum_variant as *const str as *const Self) },
                "status" => unsafe { &*(enum_variant as *const str as *const Self) },
                "statusArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &ContainerSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerSerializableScalarFields"))
        }
    }
}
/// ## Container relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct ContainerRelations {
    inner: String,
}

impl ContainerRelations {
}

impl From<ContainerRelations> for Value {
    fn from(value: ContainerRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for ContainerRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &ContainerRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerRelations"))
        }
    }
}

impl AsInterface for ContainerRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerRelations"))
        }
    }
}

impl AsInterfaceRef for ContainerRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerRelations"))
        }
    }
}
/// ## Container direct relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct ContainerDirectRelations {
    inner: String,
}

impl ContainerDirectRelations {
}

impl From<ContainerDirectRelations> for Value {
    fn from(value: ContainerDirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for ContainerDirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerDirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &ContainerDirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerDirectRelations"))
        }
    }
}

impl AsInterface for ContainerDirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerDirectRelations"))
        }
    }
}

impl AsInterfaceRef for ContainerDirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerDirectRelations"))
        }
    }
}
/// ## Container indirect relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct ContainerIndirectRelations {
    inner: String,
}

impl ContainerIndirectRelations {
}

impl From<ContainerIndirectRelations> for Value {
    fn from(value: ContainerIndirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for ContainerIndirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerIndirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &ContainerIndirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerIndirectRelations"))
        }
    }
}

impl AsInterface for ContainerIndirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to ContainerIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to ContainerIndirectRelations"))
        }
    }
}

impl AsInterfaceRef for ContainerIndirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &ContainerIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &ContainerIndirectRelations"))
        }
    }
}

/// ## Container
///
/// This model doesn't have a description.
pub struct ContainerModel {
    ctx: model::Ctx,
}

impl ContainerModel {
    /// Find many container objects.
    pub async fn find_many_objects(&self, query: impl Borrow<Value>) -> Result<Vec<Container>> {
        Ok(self.ctx.find_many(query.borrow()).await?)
    }

    /// Find a unique container object.
    pub async fn find_unique_object(&self, query: impl Borrow<Value>) -> Result<Option<Container>> {
        Ok(self.ctx.find_unique(query.borrow()).await?)
    }

    /// Find a container object.
    pub async fn find_first_object(&self, query: impl Borrow<Value>) -> Result<Option<Container>> {
        Ok(self.ctx.find_first(query.borrow()).await?)
    }

    /// Create a new container object.
    pub async fn create_object(&self, values: impl Borrow<Value>) -> Result<Container> {
        Ok(self.ctx.create_object::<Container>(values.borrow()).await?)
    }

    /// Create an empty container object.
    pub async fn create_default_object(&self) -> Result<Container> {
        Ok(self.ctx.create_object::<Container>(teon!({}).borrow()).await?)
    }

    /// Count objects on container.
    pub async fn count_objects(&self, query: impl Borrow<Value>) -> Result<usize> {
        Ok(self.ctx.count_objects(query.borrow()).await?)
    }

    /// Count fields on container.
    pub async fn count_fields(&self, query: impl Borrow<Value>) -> Result<ContainerCountAggregateResult> {
        Ok(ContainerCountAggregateResult::from_value(self.ctx.count_fields(query.borrow()).await?)?)
    }

    /// Aggregate on container.
    pub async fn aggregate(&self, query: impl Borrow<Value>) -> Result<ContainerAggregateResult> {
        Ok(ContainerAggregateResult::from_value(self.ctx.aggregate(query.borrow()).await?)?)
    }

    /// Group by on container.
    pub async fn group_by(&self, query: impl Borrow<Value>) -> Result<Vec<ContainerAggregateResult>> {
        let values: Vec<Value> = self.ctx.group_by(query.borrow()).await?;
        let mut result = vec![];
        for value in values.into_iter() {
            result.push(ContainerAggregateResult::from_value(value)?);
        }
        Ok(result)
    }

    
    /// Run a custom SQL clause.
    pub async fn sql<T, E>(&self, sql: &str) -> Result<Vec<T>> where T: TryFrom<Value, Error=E>, Error: From<E> {
        self.ctx.sql(sql).await
    }
    
}

#[derive(Clone)]
pub struct Container {
    inner: model::Object,
}

impl Container {

    /// Whether this container is new.
    pub fn is_new(&self) -> bool {
        self.inner.is_new()
    }

    /// Whether this container is modified.
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }

    /// Set new values to a container. Validations and transformations are
    /// triggered.
    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.set_teon(values.as_ref()).await
    }

    /// Update with new values to a container. Validations and transformations are
    /// not triggered.
    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.update_teon(values.as_ref()).await
    }

    /// Save this container.
    pub async fn save(&self) -> Result<()> {
        self.inner.save().await
    }

    /// Delete this container.
    pub async fn delete(&self) -> Result<()> {
        self.inner.delete().await
    }

    /// Convert this container object to teon.
    pub async fn to_teon(&self) -> Result<Value> {
        self.inner.to_teon().await
    }
    /// ## Id
    ///
    /// This field doesn't have a description.
    pub fn id(&self) -> Result<i32> {
        self.inner.get("id")
    }

    /// ## Id
    ///
    /// This field doesn't have a description.
    pub fn set_id(&self, new_value: i32) -> Result<()> {
        self.inner.set("id", new_value)
    }
    /// ## Int32
    ///
    /// This field doesn't have a description.
    pub fn int_32(&self) -> Result<Option<i32>> {
        self.inner.get("int32")
    }

    /// ## Int32
    ///
    /// This field doesn't have a description.
    pub fn set_int_32(&self, new_value: Option<i32>) -> Result<()> {
        self.inner.set("int32", new_value)
    }
    /// ## Int64
    ///
    /// This field doesn't have a description.
    pub fn int_64(&self) -> Result<Option<i64>> {
        self.inner.get("int64")
    }

    /// ## Int64
    ///
    /// This field doesn't have a description.
    pub fn set_int_64(&self, new_value: Option<i64>) -> Result<()> {
        self.inner.set("int64", new_value)
    }
    /// ## Float32
    ///
    /// This field doesn't have a description.
    pub fn float_32(&self) -> Result<Option<f32>> {
        self.inner.get("float32")
    }

    /// ## Float32
    ///
    /// This field doesn't have a description.
    pub fn set_float_32(&self, new_value: Option<f32>) -> Result<()> {
        self.inner.set("float32", new_value)
    }
    /// ## Float64
    ///
    /// This field doesn't have a description.
    pub fn float_64(&self) -> Result<Option<f64>> {
        self.inner.get("float64")
    }

    /// ## Float64
    ///
    /// This field doesn't have a description.
    pub fn set_float_64(&self, new_value: Option<f64>) -> Result<()> {
        self.inner.set("float64", new_value)
    }
    /// ## Bool
    ///
    /// This field doesn't have a description.
    pub fn bool(&self) -> Result<Option<bool>> {
        self.inner.get("bool")
    }

    /// ## Bool
    ///
    /// This field doesn't have a description.
    pub fn set_bool(&self, new_value: Option<bool>) -> Result<()> {
        self.inner.set("bool", new_value)
    }
    /// ## String
    ///
    /// This field doesn't have a description.
    pub fn string(&self) -> Result<Option<String>> {
        self.inner.get("string")
    }

    /// ## String
    ///
    /// This field doesn't have a description.
    pub fn set_string(&self, new_value: Option<String>) -> Result<()> {
        self.inner.set("string", new_value)
    }
    /// ## Date
    ///
    /// This field doesn't have a description.
    pub fn date(&self) -> Result<Option<NaiveDate>> {
        self.inner.get("date")
    }

    /// ## Date
    ///
    /// This field doesn't have a description.
    pub fn set_date(&self, new_value: Option<NaiveDate>) -> Result<()> {
        self.inner.set("date", new_value)
    }
    /// ## Date time
    ///
    /// This field doesn't have a description.
    pub fn date_time(&self) -> Result<Option<DateTime<Utc>>> {
        self.inner.get("dateTime")
    }

    /// ## Date time
    ///
    /// This field doesn't have a description.
    pub fn set_date_time(&self, new_value: Option<DateTime<Utc>>) -> Result<()> {
        self.inner.set("dateTime", new_value)
    }
    /// ## Decimal
    ///
    /// This field doesn't have a description.
    pub fn decimal(&self) -> Result<Option<BigDecimal>> {
        self.inner.get("decimal")
    }

    /// ## Decimal
    ///
    /// This field doesn't have a description.
    pub fn set_decimal(&self, new_value: Option<BigDecimal>) -> Result<()> {
        self.inner.set("decimal", new_value)
    }
    /// ## Status
    ///
    /// This field doesn't have a description.
    pub fn status(&self) -> Result<Option<Status>> {
        let value: Value = self.inner.get("status").unwrap();
        Ok(match value {
            Value::Null => None,
            _ => Some(value.try_into()?),
        })
    }

    /// ## Status
    ///
    /// This field doesn't have a description.
    pub fn set_status(&self, new_value: Option<Status>) -> Result<()> {
        self.inner.set("status", match new_value {
            None => Value::Null,
            Some(new_value) => Value::from(new_value),
        })
    }
    /// ## Int32 array
    ///
    /// This field doesn't have a description.
    pub fn int_32_array(&self) -> Result<Option<Vec<i32>>> {
        self.inner.get("int32Array")
    }

    /// ## Int32 array
    ///
    /// This field doesn't have a description.
    pub fn set_int_32_array(&self, new_value: Option<Vec<i32>>) -> Result<()> {
        self.inner.set("int32Array", new_value)
    }
    /// ## Int64 array
    ///
    /// This field doesn't have a description.
    pub fn int_64_array(&self) -> Result<Option<Vec<i64>>> {
        self.inner.get("int64Array")
    }

    /// ## Int64 array
    ///
    /// This field doesn't have a description.
    pub fn set_int_64_array(&self, new_value: Option<Vec<i64>>) -> Result<()> {
        self.inner.set("int64Array", new_value)
    }
    /// ## Float32 array
    ///
    /// This field doesn't have a description.
    pub fn float_32_array(&self) -> Result<Option<Vec<f32>>> {
        self.inner.get("float32Array")
    }

    /// ## Float32 array
    ///
    /// This field doesn't have a description.
    pub fn set_float_32_array(&self, new_value: Option<Vec<f32>>) -> Result<()> {
        self.inner.set("float32Array", new_value)
    }
    /// ## Float64 array
    ///
    /// This field doesn't have a description.
    pub fn float_64_array(&self) -> Result<Option<Vec<f64>>> {
        self.inner.get("float64Array")
    }

    /// ## Float64 array
    ///
    /// This field doesn't have a description.
    pub fn set_float_64_array(&self, new_value: Option<Vec<f64>>) -> Result<()> {
        self.inner.set("float64Array", new_value)
    }
    /// ## Bool array
    ///
    /// This field doesn't have a description.
    pub fn bool_array(&self) -> Result<Option<Vec<bool>>> {
        self.inner.get("boolArray")
    }

    /// ## Bool array
    ///
    /// This field doesn't have a description.
    pub fn set_bool_array(&self, new_value: Option<Vec<bool>>) -> Result<()> {
        self.inner.set("boolArray", new_value)
    }
    /// ## String array
    ///
    /// This field doesn't have a description.
    pub fn string_array(&self) -> Result<Option<Vec<String>>> {
        self.inner.get("stringArray")
    }

    /// ## String array
    ///
    /// This field doesn't have a description.
    pub fn set_string_array(&self, new_value: Option<Vec<String>>) -> Result<()> {
        self.inner.set("stringArray", new_value)
    }
    /// ## Date array
    ///
    /// This field doesn't have a description.
    pub fn date_array(&self) -> Result<Option<Vec<NaiveDate>>> {
        self.inner.get("dateArray")
    }

    /// ## Date array
    ///
    /// This field doesn't have a description.
    pub fn set_date_array(&self, new_value: Option<Vec<NaiveDate>>) -> Result<()> {
        self.inner.set("dateArray", new_value)
    }
    /// ## Date time array
    ///
    /// This field doesn't have a description.
    pub fn date_time_array(&self) -> Result<Option<Vec<DateTime<Utc>>>> {
        self.inner.get("dateTimeArray")
    }

    /// ## Date time array
    ///
    /// This field doesn't have a description.
    pub fn set_date_time_array(&self, new_value: Option<Vec<DateTime<Utc>>>) -> Result<()> {
        self.inner.set("dateTimeArray", new_value)
    }
    /// ## Decimal array
    ///
    /// This field doesn't have a description.
    pub fn decimal_array(&self) -> Result<Option<Vec<BigDecimal>>> {
        self.inner.get("decimalArray")
    }

    /// ## Decimal array
    ///
    /// This field doesn't have a description.
    pub fn set_decimal_array(&self, new_value: Option<Vec<BigDecimal>>) -> Result<()> {
        self.inner.set("decimalArray", new_value)
    }
    /// ## Status array
    ///
    /// This field doesn't have a description.
    pub fn status_array(&self) -> Result<Option<Vec<Status>>> {
        self.inner.get("statusArray")
    }

    /// ## Status array
    ///
    /// This field doesn't have a description.
    pub fn set_status_array(&self, new_value: Option<Vec<Status>>) -> Result<()> {
        self.inner.set("statusArray", new_value)
    }
    /// ## Message
    ///
    /// This field doesn't have a description.
    pub fn message(&self) -> Result<Option<String>> {
        self.inner.get("message")
    }

    /// ## Message
    ///
    /// This field doesn't have a description.
    pub fn set_message(&self, new_value: Option<String>) -> Result<()> {
        self.inner.set("message", new_value)
    }
}

impl From<Container> for model::Object {
    fn from(value: Container) -> Self {
        value.inner.clone()
    }
}

impl From<model::Object> for Container {
    fn from(value: model::Object) -> Self {
        Self { inner: value }
    }
}

impl From<Container> for Value {
    fn from(value: Container) -> Self {
        Value::ModelObject(value.inner.clone())
    }
}

impl AsInterface for Container {
    fn from_value(value: Value) -> Result<Self> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl TryFrom<Value> for Container {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl Debug for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ExtractFromPipelineCtx for Container {
    fn extract(ctx: &pipeline::Ctx) -> Self {
        Container {
            inner: ctx.object().clone(),
        }
    }
}

pub trait ContainerSelectTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerSelect {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSelect {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSelectTrait for ContainerSelect { }

impl AsInterface for ContainerSelect {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSelect> for Value {
    fn from(value: ContainerSelect) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSelect {

    fn from_value_ref(value: &Value) -> Result<&ContainerSelect> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSelect)
        })
    }
}

impl ExtractFromRequest for ContainerSelect {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerIncludeTrait: Interface {
}

#[repr(transparent)]
pub struct ContainerInclude {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerInclude {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerIncludeTrait for ContainerInclude { }

impl AsInterface for ContainerInclude {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerInclude> for Value {
    fn from(value: ContainerInclude) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerInclude {

    fn from_value_ref(value: &Value) -> Result<&ContainerInclude> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerInclude)
        })
    }
}

impl ExtractFromRequest for ContainerInclude {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerWhereInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&ContainerWhereInput>> {
        Some(Vec::<&ContainerWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<ContainerWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("AND".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("AND");
            },
        }
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn not(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("NOT".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("NOT");
            },
        }
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn or(&self) -> Option<Vec<&ContainerWhereInput>> {
        Some(Vec::<&ContainerWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<ContainerWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("OR".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("OR");
            },
        }
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerWhereInputTrait for ContainerWhereInput { }

impl AsInterface for ContainerWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerWhereInput> for Value {
    fn from(value: ContainerWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerWhereInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerWhereInput)
        })
    }
}

impl ExtractFromRequest for ContainerWhereInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerWhereUniqueInputTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> &i32 {
        i32::from_value_ref(self.inner().get("id").unwrap()).unwrap()
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: i32) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerWhereUniqueInputTrait for ContainerWhereUniqueInput { }

impl AsInterface for ContainerWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerWhereUniqueInput> for Value {
    fn from(value: ContainerWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerWhereUniqueInput)
        })
    }
}

impl ExtractFromRequest for ContainerWhereUniqueInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerScalarWhereWithAggregatesInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&ContainerWhereInput>> {
        Some(Vec::<&ContainerWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<ContainerWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("AND".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("AND");
            },
        }
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn not(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("NOT".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("NOT");
            },
        }
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn or(&self) -> Option<Vec<&ContainerWhereInput>> {
        Some(Vec::<&ContainerWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<ContainerWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("OR".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("OR");
            },
        }
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerScalarWhereWithAggregatesInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerScalarWhereWithAggregatesInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerScalarWhereWithAggregatesInputTrait for ContainerScalarWhereWithAggregatesInput { }

impl AsInterface for ContainerScalarWhereWithAggregatesInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerScalarWhereWithAggregatesInput> for Value {
    fn from(value: ContainerScalarWhereWithAggregatesInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerScalarWhereWithAggregatesInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerScalarWhereWithAggregatesInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerScalarWhereWithAggregatesInput)
        })
    }
}

impl ExtractFromRequest for ContainerScalarWhereWithAggregatesInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerRelationFilterTrait: Interface {
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn is(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("is")?).unwrap())
    }
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn set_is(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("is".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("is");
            },
        }
    }
    /// ## Is Not
    ///
    /// This synthesized field doesn't have a description.
    fn is_not(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("isNot")?).unwrap())
    }
    /// ## Is Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_is_not(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("isNot".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("isNot");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerRelationFilterTrait for ContainerRelationFilter { }

impl AsInterface for ContainerRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerRelationFilter> for Value {
    fn from(value: ContainerRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&ContainerRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerRelationFilter)
        })
    }
}

impl ExtractFromRequest for ContainerRelationFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerListRelationFilterTrait: Interface {
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn every(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("every")?).unwrap())
    }
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn set_every(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("every".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("every");
            },
        }
    }
    /// ## None
    ///
    /// This synthesized field doesn't have a description.
    fn none(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("none")?).unwrap())
    }
    /// ## None
    ///
    /// This synthesized field doesn't have a description.
    fn set_none(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("none".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("none");
            },
        }
    }
    /// ## Some
    ///
    /// This synthesized field doesn't have a description.
    fn some(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("some")?).unwrap())
    }
    /// ## Some
    ///
    /// This synthesized field doesn't have a description.
    fn set_some(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("some".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("some");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerListRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerListRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerListRelationFilterTrait for ContainerListRelationFilter { }

impl AsInterface for ContainerListRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerListRelationFilter> for Value {
    fn from(value: ContainerListRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerListRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&ContainerListRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerListRelationFilter)
        })
    }
}

impl ExtractFromRequest for ContainerListRelationFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerOrderByInputTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerOrderByInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerOrderByInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerOrderByInputTrait for ContainerOrderByInput { }

impl AsInterface for ContainerOrderByInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerOrderByInput> for Value {
    fn from(value: ContainerOrderByInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerOrderByInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerOrderByInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerOrderByInput)
        })
    }
}

impl ExtractFromRequest for ContainerOrderByInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCountAggregateInputTypeTrait: Interface {
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn all(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("_all")?).unwrap())
    }
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn set_all(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_all".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_all");
            },
        }
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCountAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCountAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCountAggregateInputTypeTrait for ContainerCountAggregateInputType { }

impl AsInterface for ContainerCountAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCountAggregateInputType> for Value {
    fn from(value: ContainerCountAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCountAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&ContainerCountAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCountAggregateInputType)
        })
    }
}

impl ExtractFromRequest for ContainerCountAggregateInputType {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSumAggregateInputTypeTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerSumAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSumAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSumAggregateInputTypeTrait for ContainerSumAggregateInputType { }

impl AsInterface for ContainerSumAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSumAggregateInputType> for Value {
    fn from(value: ContainerSumAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSumAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&ContainerSumAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSumAggregateInputType)
        })
    }
}

impl ExtractFromRequest for ContainerSumAggregateInputType {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerAvgAggregateInputTypeTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerAvgAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerAvgAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerAvgAggregateInputTypeTrait for ContainerAvgAggregateInputType { }

impl AsInterface for ContainerAvgAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerAvgAggregateInputType> for Value {
    fn from(value: ContainerAvgAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerAvgAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&ContainerAvgAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerAvgAggregateInputType)
        })
    }
}

impl ExtractFromRequest for ContainerAvgAggregateInputType {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerMinAggregateInputTypeTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerMinAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerMinAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerMinAggregateInputTypeTrait for ContainerMinAggregateInputType { }

impl AsInterface for ContainerMinAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerMinAggregateInputType> for Value {
    fn from(value: ContainerMinAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerMinAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&ContainerMinAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerMinAggregateInputType)
        })
    }
}

impl ExtractFromRequest for ContainerMinAggregateInputType {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerMaxAggregateInputTypeTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerMaxAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerMaxAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerMaxAggregateInputTypeTrait for ContainerMaxAggregateInputType { }

impl AsInterface for ContainerMaxAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerMaxAggregateInputType> for Value {
    fn from(value: ContainerMaxAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerMaxAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&ContainerMaxAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerMaxAggregateInputType)
        })
    }
}

impl ExtractFromRequest for ContainerMaxAggregateInputType {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCreateInputTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCreateInputTrait for ContainerCreateInput { }

impl AsInterface for ContainerCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCreateInput> for Value {
    fn from(value: ContainerCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCreateInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCreateInput)
        })
    }
}

impl ExtractFromRequest for ContainerCreateInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateInputTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateInputTrait for ContainerUpdateInput { }

impl AsInterface for ContainerUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateInput> for Value {
    fn from(value: ContainerUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCreateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&ContainerConnectOrCreateInput> {
        Some(ContainerConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<ContainerConnectOrCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&ContainerCreateInput> {
        Some(ContainerCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<ContainerCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCreateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCreateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCreateNestedOneInputTrait for ContainerCreateNestedOneInput { }

impl AsInterface for ContainerCreateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCreateNestedOneInput> for Value {
    fn from(value: ContainerCreateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCreateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerCreateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCreateNestedOneInput)
        })
    }
}

impl ExtractFromRequest for ContainerCreateNestedOneInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCreateNestedManyInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCreateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCreateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCreateNestedManyInputTrait for ContainerCreateNestedManyInput { }

impl AsInterface for ContainerCreateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCreateNestedManyInput> for Value {
    fn from(value: ContainerCreateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCreateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerCreateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCreateNestedManyInput)
        })
    }
}

impl ExtractFromRequest for ContainerCreateNestedManyInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&ContainerConnectOrCreateInput> {
        Some(ContainerConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<ContainerConnectOrCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&ContainerCreateInput> {
        Some(ContainerCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<ContainerCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn delete(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("delete")?).unwrap())
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("delete".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("delete");
            },
        }
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn disconnect(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("disconnect")?).unwrap())
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn set_disconnect(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("disconnect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("disconnect");
            },
        }
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("set")?).unwrap())
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set_set(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("set".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("set");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> Option<&ContainerUpdateInput> {
        Some(ContainerUpdateInput::from_value_ref(self.inner().get("update")?).unwrap())
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: Option<ContainerUpdateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("update");
            },
        }
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn upsert(&self) -> Option<&ContainerUpsertWithWhereUniqueInput> {
        Some(ContainerUpsertWithWhereUniqueInput::from_value_ref(self.inner().get("upsert")?).unwrap())
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn set_upsert(&mut self, new_value: Option<ContainerUpsertWithWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("upsert".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("upsert");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerUpdateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateNestedOneInputTrait for ContainerUpdateNestedOneInput { }

impl AsInterface for ContainerUpdateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateNestedOneInput> for Value {
    fn from(value: ContainerUpdateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateNestedOneInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateNestedOneInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateNestedManyInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn delete(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("delete")?).unwrap())
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("delete".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("delete");
            },
        }
    }
    /// ## Delete Many
    ///
    /// This synthesized field doesn't have a description.
    fn delete_many(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("deleteMany")?).unwrap())
    }
    /// ## Delete Many
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete_many(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("deleteMany".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("deleteMany");
            },
        }
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn disconnect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("disconnect")?).unwrap())
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn set_disconnect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("disconnect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("disconnect");
            },
        }
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("set")?).unwrap())
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set_set(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("set".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("set");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("update")?).unwrap())
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("update");
            },
        }
    }
    /// ## Update Many
    ///
    /// This synthesized field doesn't have a description.
    fn update_many(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("updateMany")?).unwrap())
    }
    /// ## Update Many
    ///
    /// This synthesized field doesn't have a description.
    fn set_update_many(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updateMany".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updateMany");
            },
        }
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn upsert(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("upsert")?).unwrap())
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn set_upsert(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("upsert".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("upsert");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerUpdateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateNestedManyInputTrait for ContainerUpdateNestedManyInput { }

impl AsInterface for ContainerUpdateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateNestedManyInput> for Value {
    fn from(value: ContainerUpdateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateNestedManyInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateNestedManyInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerConnectOrCreateInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &ContainerCreateInput {
        ContainerCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: ContainerCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerConnectOrCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerConnectOrCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerConnectOrCreateInputTrait for ContainerConnectOrCreateInput { }

impl AsInterface for ContainerConnectOrCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerConnectOrCreateInput> for Value {
    fn from(value: ContainerConnectOrCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerConnectOrCreateInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerConnectOrCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerConnectOrCreateInput)
        })
    }
}

impl ExtractFromRequest for ContainerConnectOrCreateInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateWithWhereUniqueInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpdateWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateWithWhereUniqueInputTrait for ContainerUpdateWithWhereUniqueInput { }

impl AsInterface for ContainerUpdateWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateWithWhereUniqueInput> for Value {
    fn from(value: ContainerUpdateWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateWithWhereUniqueInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateWithWhereUniqueInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpsertWithWhereUniqueInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &ContainerCreateInput {
        ContainerCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: ContainerCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpsertWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpsertWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpsertWithWhereUniqueInputTrait for ContainerUpsertWithWhereUniqueInput { }

impl AsInterface for ContainerUpsertWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpsertWithWhereUniqueInput> for Value {
    fn from(value: ContainerUpsertWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpsertWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpsertWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpsertWithWhereUniqueInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpsertWithWhereUniqueInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateManyWithWhereInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereInput {
        ContainerWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpdateManyWithWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateManyWithWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateManyWithWhereInputTrait for ContainerUpdateManyWithWhereInput { }

impl AsInterface for ContainerUpdateManyWithWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateManyWithWhereInput> for Value {
    fn from(value: ContainerUpdateManyWithWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateManyWithWhereInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateManyWithWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateManyWithWhereInput)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateManyWithWhereInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerResultTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> &i32 {
        i32::from_value_ref(self.inner().get("id").unwrap()).unwrap()
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: i32) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into()).unwrap();
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerResultTrait for ContainerResult { }

impl AsInterface for ContainerResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerResult> for Value {
    fn from(value: ContainerResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerResult)
        })
    }
}

impl ExtractFromRequest for ContainerResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCountAggregateResultTrait: Interface {
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn all(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_all")?).unwrap())
    }
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn set_all(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_all".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_all");
            },
        }
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCountAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCountAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCountAggregateResultTrait for ContainerCountAggregateResult { }

impl AsInterface for ContainerCountAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCountAggregateResult> for Value {
    fn from(value: ContainerCountAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCountAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerCountAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCountAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerCountAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSumAggregateResultTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerSumAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSumAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSumAggregateResultTrait for ContainerSumAggregateResult { }

impl AsInterface for ContainerSumAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSumAggregateResult> for Value {
    fn from(value: ContainerSumAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSumAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerSumAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSumAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerSumAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerAvgAggregateResultTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerAvgAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerAvgAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerAvgAggregateResultTrait for ContainerAvgAggregateResult { }

impl AsInterface for ContainerAvgAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerAvgAggregateResult> for Value {
    fn from(value: ContainerAvgAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerAvgAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerAvgAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerAvgAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerAvgAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerMinAggregateResultTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerMinAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerMinAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerMinAggregateResultTrait for ContainerMinAggregateResult { }

impl AsInterface for ContainerMinAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerMinAggregateResult> for Value {
    fn from(value: ContainerMinAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerMinAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerMinAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerMinAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerMinAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerMaxAggregateResultTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerMaxAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerMaxAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerMaxAggregateResultTrait for ContainerMaxAggregateResult { }

impl AsInterface for ContainerMaxAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerMaxAggregateResult> for Value {
    fn from(value: ContainerMaxAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerMaxAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerMaxAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerMaxAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerMaxAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerAggregateResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&ContainerAvgAggregateResult> {
        Some(ContainerAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<ContainerAvgAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&ContainerCountAggregateResult> {
        Some(ContainerCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<ContainerCountAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&ContainerMaxAggregateResult> {
        Some(ContainerMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ContainerMaxAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&ContainerMinAggregateResult> {
        Some(ContainerMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ContainerMinAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&ContainerSumAggregateResult> {
        Some(ContainerSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<ContainerSumAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerAggregateResultTrait for ContainerAggregateResult { }

impl AsInterface for ContainerAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerAggregateResult> for Value {
    fn from(value: ContainerAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerAggregateResult)
        })
    }
}

impl ExtractFromRequest for ContainerAggregateResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerGroupByResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&ContainerAvgAggregateResult> {
        Some(ContainerAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<ContainerAvgAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&ContainerCountAggregateResult> {
        Some(ContainerCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<ContainerCountAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&ContainerMaxAggregateResult> {
        Some(ContainerMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ContainerMaxAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&ContainerMinAggregateResult> {
        Some(ContainerMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ContainerMinAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&ContainerSumAggregateResult> {
        Some(ContainerSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<ContainerSumAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerGroupByResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerGroupByResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerGroupByResultTrait for ContainerGroupByResult { }

impl AsInterface for ContainerGroupByResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerGroupByResult> for Value {
    fn from(value: ContainerGroupByResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerGroupByResult {

    fn from_value_ref(value: &Value) -> Result<&ContainerGroupByResult> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerGroupByResult)
        })
    }
}

impl ExtractFromRequest for ContainerGroupByResult {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerArgsTrait for ContainerArgs { }

impl AsInterface for ContainerArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerArgs> for Value {
    fn from(value: ContainerArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerArgs)
        })
    }
}

impl ExtractFromRequest for ContainerArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerFindManyArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&ContainerSerializableScalarFields> {
        Some(ContainerSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<ContainerSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerFindManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerFindManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerFindManyArgsTrait for ContainerFindManyArgs { }

impl AsInterface for ContainerFindManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerFindManyArgs> for Value {
    fn from(value: ContainerFindManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerFindManyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerFindManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerFindManyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerFindManyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerFindFirstArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&ContainerSerializableScalarFields> {
        Some(ContainerSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<ContainerSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerFindFirstArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerFindFirstArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerFindFirstArgsTrait for ContainerFindFirstArgs { }

impl AsInterface for ContainerFindFirstArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerFindFirstArgs> for Value {
    fn from(value: ContainerFindFirstArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerFindFirstArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerFindFirstArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerFindFirstArgs)
        })
    }
}

impl ExtractFromRequest for ContainerFindFirstArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerFindUniqueArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerFindUniqueArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerFindUniqueArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerFindUniqueArgsTrait for ContainerFindUniqueArgs { }

impl AsInterface for ContainerFindUniqueArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerFindUniqueArgs> for Value {
    fn from(value: ContainerFindUniqueArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerFindUniqueArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerFindUniqueArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerFindUniqueArgs)
        })
    }
}

impl ExtractFromRequest for ContainerFindUniqueArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCreateArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &ContainerCreateInput {
        ContainerCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: ContainerCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCreateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCreateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCreateArgsTrait for ContainerCreateArgs { }

impl AsInterface for ContainerCreateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCreateArgs> for Value {
    fn from(value: ContainerCreateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCreateArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerCreateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCreateArgs)
        })
    }
}

impl ExtractFromRequest for ContainerCreateArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpdateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateArgsTrait for ContainerUpdateArgs { }

impl AsInterface for ContainerUpdateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateArgs> for Value {
    fn from(value: ContainerUpdateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateArgs)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpsertArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &ContainerCreateInput {
        ContainerCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: ContainerCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpsertArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpsertArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpsertArgsTrait for ContainerUpsertArgs { }

impl AsInterface for ContainerUpsertArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpsertArgs> for Value {
    fn from(value: ContainerUpsertArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpsertArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpsertArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpsertArgs)
        })
    }
}

impl ExtractFromRequest for ContainerUpsertArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCopyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerCopyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCopyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCopyArgsTrait for ContainerCopyArgs { }

impl AsInterface for ContainerCopyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCopyArgs> for Value {
    fn from(value: ContainerCopyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCopyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerCopyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCopyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerCopyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerDeleteArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereUniqueInput {
        ContainerWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerDeleteArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerDeleteArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerDeleteArgsTrait for ContainerDeleteArgs { }

impl AsInterface for ContainerDeleteArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerDeleteArgs> for Value {
    fn from(value: ContainerDeleteArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerDeleteArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerDeleteArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerDeleteArgs)
        })
    }
}

impl ExtractFromRequest for ContainerDeleteArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCreateManyArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &Value {
        Value::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Value) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCreateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCreateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCreateManyArgsTrait for ContainerCreateManyArgs { }

impl AsInterface for ContainerCreateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCreateManyArgs> for Value {
    fn from(value: ContainerCreateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCreateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerCreateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCreateManyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerCreateManyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerUpdateManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereInput {
        ContainerWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerUpdateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerUpdateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerUpdateManyArgsTrait for ContainerUpdateManyArgs { }

impl AsInterface for ContainerUpdateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerUpdateManyArgs> for Value {
    fn from(value: ContainerUpdateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerUpdateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerUpdateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerUpdateManyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerUpdateManyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerDeleteManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereInput {
        ContainerWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerDeleteManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerDeleteManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerDeleteManyArgsTrait for ContainerDeleteManyArgs { }

impl AsInterface for ContainerDeleteManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerDeleteManyArgs> for Value {
    fn from(value: ContainerDeleteManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerDeleteManyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerDeleteManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerDeleteManyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerDeleteManyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCopyManyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &ContainerUpdateInput {
        ContainerUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: ContainerUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &ContainerWhereInput {
        ContainerWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: ContainerWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ContainerCopyManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCopyManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCopyManyArgsTrait for ContainerCopyManyArgs { }

impl AsInterface for ContainerCopyManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCopyManyArgs> for Value {
    fn from(value: ContainerCopyManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCopyManyArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerCopyManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCopyManyArgs)
        })
    }
}

impl ExtractFromRequest for ContainerCopyManyArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerCountArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&ContainerSerializableScalarFields> {
        Some(ContainerSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<ContainerSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerCountAggregateInputType> {
        Some(ContainerCountAggregateInputType::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerCountArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerCountArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerCountArgsTrait for ContainerCountArgs { }

impl AsInterface for ContainerCountArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerCountArgs> for Value {
    fn from(value: ContainerCountArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerCountArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerCountArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerCountArgs)
        })
    }
}

impl ExtractFromRequest for ContainerCountArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerAggregateArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&ContainerAvgAggregateInputType> {
        Some(ContainerAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<ContainerAvgAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&ContainerCountAggregateInputType> {
        Some(ContainerCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<ContainerCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&ContainerMaxAggregateInputType> {
        Some(ContainerMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ContainerMaxAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&ContainerMinAggregateInputType> {
        Some(ContainerMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ContainerMinAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&ContainerSumAggregateInputType> {
        Some(ContainerSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<ContainerSumAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&ContainerSerializableScalarFields> {
        Some(ContainerSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<ContainerSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerAggregateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerAggregateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerAggregateArgsTrait for ContainerAggregateArgs { }

impl AsInterface for ContainerAggregateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerAggregateArgs> for Value {
    fn from(value: ContainerAggregateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerAggregateArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerAggregateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerAggregateArgs)
        })
    }
}

impl ExtractFromRequest for ContainerAggregateArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerGroupByArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&ContainerAvgAggregateInputType> {
        Some(ContainerAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<ContainerAvgAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&ContainerCountAggregateInputType> {
        Some(ContainerCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<ContainerCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&ContainerMaxAggregateInputType> {
        Some(ContainerMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ContainerMaxAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&ContainerMinAggregateInputType> {
        Some(ContainerMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ContainerMinAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&ContainerSumAggregateInputType> {
        Some(ContainerSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<ContainerSumAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## By
    ///
    /// This synthesized field doesn't have a description.
    fn by(&self) -> &Value {
        Value::from_value_ref(self.inner().get("by").unwrap()).unwrap()
    }
    /// ## By
    ///
    /// This synthesized field doesn't have a description.
    fn set_by(&mut self, new_value: Value) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("by".to_owned(), new_value.into()).unwrap();
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&ContainerWhereUniqueInput> {
        Some(ContainerWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<ContainerWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&ContainerSerializableScalarFields> {
        Some(ContainerSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<ContainerSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Having
    ///
    /// This synthesized field doesn't have a description.
    fn having(&self) -> Option<&ContainerScalarWhereWithAggregatesInput> {
        Some(ContainerScalarWhereWithAggregatesInput::from_value_ref(self.inner().get("having")?).unwrap())
    }
    /// ## Having
    ///
    /// This synthesized field doesn't have a description.
    fn set_having(&mut self, new_value: Option<ContainerScalarWhereWithAggregatesInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("having".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("having");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&ContainerWhereInput> {
        Some(ContainerWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<ContainerWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerGroupByArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerGroupByArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerGroupByArgsTrait for ContainerGroupByArgs { }

impl AsInterface for ContainerGroupByArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerGroupByArgs> for Value {
    fn from(value: ContainerGroupByArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerGroupByArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerGroupByArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerGroupByArgs)
        })
    }
}

impl ExtractFromRequest for ContainerGroupByArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerScalarUpdateInputTrait: Interface {
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn bool(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("bool")?).unwrap())
    }
    /// ## Bool
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("bool".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("bool");
            },
        }
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn bool_array(&self) -> Option<Vec<&bool>> {
        Some(Vec::<&bool>::from_value_ref_vec(self.inner().get("boolArray")?).unwrap())
    }
    /// ## Bool Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_bool_array(&mut self, new_value: Option<Vec<bool>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("boolArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("boolArray");
            },
        }
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn date(&self) -> Option<&NaiveDate> {
        Some(NaiveDate::from_value_ref(self.inner().get("date")?).unwrap())
    }
    /// ## Date
    ///
    /// This synthesized field doesn't have a description.
    fn set_date(&mut self, new_value: Option<NaiveDate>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("date".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("date");
            },
        }
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_array(&self) -> Option<Vec<&NaiveDate>> {
        Some(Vec::<&NaiveDate>::from_value_ref_vec(self.inner().get("dateArray")?).unwrap())
    }
    /// ## Date Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_array(&mut self, new_value: Option<Vec<NaiveDate>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateArray");
            },
        }
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn date_time(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("dateTime")?).unwrap())
    }
    /// ## Date Time
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTime".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTime");
            },
        }
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn date_time_array(&self) -> Option<Vec<&DateTime<Utc>>> {
        Some(Vec::<&DateTime<Utc>>::from_value_ref_vec(self.inner().get("dateTimeArray")?).unwrap())
    }
    /// ## Date Time Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_date_time_array(&mut self, new_value: Option<Vec<DateTime<Utc>>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("dateTimeArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("dateTimeArray");
            },
        }
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn decimal(&self) -> Option<&BigDecimal> {
        Some(BigDecimal::from_value_ref(self.inner().get("decimal")?).unwrap())
    }
    /// ## Decimal
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal(&mut self, new_value: Option<BigDecimal>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimal".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimal");
            },
        }
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn decimal_array(&self) -> Option<Vec<&BigDecimal>> {
        Some(Vec::<&BigDecimal>::from_value_ref_vec(self.inner().get("decimalArray")?).unwrap())
    }
    /// ## Decimal Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_decimal_array(&mut self, new_value: Option<Vec<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decimalArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decimalArray");
            },
        }
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn float_32(&self) -> Option<&f32> {
        Some(f32::from_value_ref(self.inner().get("float32")?).unwrap())
    }
    /// ## Float32
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32(&mut self, new_value: Option<f32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32");
            },
        }
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_32_array(&self) -> Option<Vec<&f32>> {
        Some(Vec::<&f32>::from_value_ref_vec(self.inner().get("float32Array")?).unwrap())
    }
    /// ## Float32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_32_array(&mut self, new_value: Option<Vec<f32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float32Array");
            },
        }
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn float_64(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("float64")?).unwrap())
    }
    /// ## Float64
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64");
            },
        }
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn float_64_array(&self) -> Option<Vec<&f64>> {
        Some(Vec::<&f64>::from_value_ref_vec(self.inner().get("float64Array")?).unwrap())
    }
    /// ## Float64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_float_64_array(&mut self, new_value: Option<Vec<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("float64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("float64Array");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn int_32(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("int32")?).unwrap())
    }
    /// ## Int32
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32");
            },
        }
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_32_array(&self) -> Option<Vec<&i32>> {
        Some(Vec::<&i32>::from_value_ref_vec(self.inner().get("int32Array")?).unwrap())
    }
    /// ## Int32 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_32_array(&mut self, new_value: Option<Vec<i32>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int32Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int32Array");
            },
        }
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn int_64(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("int64")?).unwrap())
    }
    /// ## Int64
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64");
            },
        }
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn int_64_array(&self) -> Option<Vec<&i64>> {
        Some(Vec::<&i64>::from_value_ref_vec(self.inner().get("int64Array")?).unwrap())
    }
    /// ## Int64 Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_int_64_array(&mut self, new_value: Option<Vec<i64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("int64Array".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("int64Array");
            },
        }
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn message(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("message")?).unwrap())
    }
    /// ## Message
    ///
    /// This synthesized field doesn't have a description.
    fn set_message(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("message");
            },
        }
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn status(&self) -> Option<&Status> {
        Some(Status::from_value_ref(self.inner().get("status")?).unwrap())
    }
    /// ## Status
    ///
    /// This synthesized field doesn't have a description.
    fn set_status(&mut self, new_value: Option<Status>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("status".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("status");
            },
        }
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn status_array(&self) -> Option<Vec<&Status>> {
        Some(Vec::<&Status>::from_value_ref_vec(self.inner().get("statusArray")?).unwrap())
    }
    /// ## Status Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_status_array(&mut self, new_value: Option<Vec<Status>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("statusArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("statusArray");
            },
        }
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn string(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("string")?).unwrap())
    }
    /// ## String
    ///
    /// This synthesized field doesn't have a description.
    fn set_string(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("string".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("string");
            },
        }
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn string_array(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("stringArray")?).unwrap())
    }
    /// ## String Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_string_array(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("stringArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("stringArray");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerScalarUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerScalarUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerScalarUpdateInputTrait for ContainerScalarUpdateInput { }

impl AsInterface for ContainerScalarUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerScalarUpdateInput> for Value {
    fn from(value: ContainerScalarUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerScalarUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerScalarUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerScalarUpdateInput)
        })
    }
}

impl ExtractFromRequest for ContainerScalarUpdateInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSignInCheckerIdsTrait: Interface {
}

#[repr(transparent)]
pub struct ContainerSignInCheckerIds {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSignInCheckerIds {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSignInCheckerIdsTrait for ContainerSignInCheckerIds { }

impl AsInterface for ContainerSignInCheckerIds {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSignInCheckerIds> for Value {
    fn from(value: ContainerSignInCheckerIds) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSignInCheckerIds {

    fn from_value_ref(value: &Value) -> Result<&ContainerSignInCheckerIds> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSignInCheckerIds)
        })
    }
}

impl ExtractFromRequest for ContainerSignInCheckerIds {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSignInCheckerCompanionsTrait: Interface {
}

#[repr(transparent)]
pub struct ContainerSignInCheckerCompanions {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSignInCheckerCompanions {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSignInCheckerCompanionsTrait for ContainerSignInCheckerCompanions { }

impl AsInterface for ContainerSignInCheckerCompanions {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSignInCheckerCompanions> for Value {
    fn from(value: ContainerSignInCheckerCompanions) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSignInCheckerCompanions {

    fn from_value_ref(value: &Value) -> Result<&ContainerSignInCheckerCompanions> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSignInCheckerCompanions)
        })
    }
}

impl ExtractFromRequest for ContainerSignInCheckerCompanions {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSignInInputTrait: Interface {
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn credentials(&self) -> &ContainerSignInArgs {
        ContainerSignInArgs::from_value_ref(self.inner().get("credentials").unwrap()).unwrap()
    }
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn set_credentials(&mut self, new_value: ContainerSignInArgs) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("credentials".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&ContainerInclude> {
        Some(ContainerInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<ContainerInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&ContainerSelect> {
        Some(ContainerSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<ContainerSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct ContainerSignInInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSignInInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSignInInputTrait for ContainerSignInInput { }

impl AsInterface for ContainerSignInInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSignInInput> for Value {
    fn from(value: ContainerSignInInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSignInInput {

    fn from_value_ref(value: &Value) -> Result<&ContainerSignInInput> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSignInInput)
        })
    }
}

impl ExtractFromRequest for ContainerSignInInput {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ContainerSignInArgsTrait: Interface {
}

#[repr(transparent)]
pub struct ContainerSignInArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ContainerSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ContainerSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ContainerSignInArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ContainerSignInArgsTrait for ContainerSignInArgs { }

impl AsInterface for ContainerSignInArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ContainerSignInArgs> for Value {
    fn from(value: ContainerSignInArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ContainerSignInArgs {

    fn from_value_ref(value: &Value) -> Result<&ContainerSignInArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const ContainerSignInArgs)
        })
    }
}

impl ExtractFromRequest for ContainerSignInArgs {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub struct Teo {
    pub(crate) ctx: transaction::Ctx,
}

impl From<transaction::Ctx> for Teo {
    fn from(value: transaction::Ctx) -> Self {
        Self { ctx: value }
    }
}

impl Teo {

    pub async fn transaction<F, C, Fut, R>(&self, f: F) -> Result<R> where
        F: Fn(C) -> Fut,
        C: for <'a> From<&'a transaction::Ctx>,
        Fut: Future<Output = Result<R>> {
        Ok(self.ctx.run_transaction(f).await?)
    }
    
    pub fn container(&self) -> ContainerModel {
        ContainerModel { ctx: self.ctx.model_ctx_for_model_at_path(&vec!["Container".to_owned()]).unwrap() }
    }
}


impl ExtractFromTransactionCtx for Teo {
    fn extract(ctx: &transaction::Ctx) -> Self {
        Teo {
            ctx: ctx.clone(),
        }
    }
}

impl ExtractFromRequest for Teo {
    fn extract(request: &Request) -> Self {
        Teo {
            ctx: request.transaction_ctx().clone(),
        }
    }
}

impl ExtractFromPipelineCtx for Teo {
    fn extract(ctx: &pipeline::Ctx) -> Self {
        Teo {
            ctx: ctx.transaction_ctx().clone(),
        }
    }
}
