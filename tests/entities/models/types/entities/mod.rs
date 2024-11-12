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


/// ## Sex
///
/// This enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct Sex {
    inner: String,
}

impl Sex {
    /// ### Is Male
    ///
    /// Returns true if value is male
    pub fn is_male(&self) -> bool {
        self.inner.as_str() == "male"
    }
    /// ### Male
    ///
    /// This enum member doesn't have a description.
    pub fn male() -> Self {
        Self { inner: "male".to_owned() }
    }
    /// ### Is Female
    ///
    /// Returns true if value is female
    pub fn is_female(&self) -> bool {
        self.inner.as_str() == "female"
    }
    /// ### Female
    ///
    /// This enum member doesn't have a description.
    pub fn female() -> Self {
        Self { inner: "female".to_owned() }
    }
}

impl From<Sex> for Value {
    fn from(value: Sex) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for Sex {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "male" => Sex::male(),
                "female" => Sex::female(),
                _ => Err(Error::new("cannot convert value to Sex"))?
            })
        } else {
            Err(Error::new("cannot convert value to Sex"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &Sex {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "male" => unsafe { &*(enum_variant as *const str as *const Self) },
                "female" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Sex"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Sex"))
        }
    }
}

impl AsInterface for Sex {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "male" => Sex::male(),
                "female" => Sex::female(),
                _ => Err(Error::new("cannot convert value to Sex"))?
            })
        } else {
            Err(Error::new("cannot convert value to Sex"))
        }
    }
}

impl AsInterfaceRef for Sex {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "male" => unsafe { &*(enum_variant as *const str as *const Self) },
                "female" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Sex"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Sex"))
        }
    }
}
/// ## Support scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct SupportScalarFields {
    inner: String,
}

impl SupportScalarFields {
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
    /// ### Is Sex
    ///
    /// Returns true if value is sex
    pub fn is_sex(&self) -> bool {
        self.inner.as_str() == "sex"
    }
    /// ### Sex
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn sex() -> Self {
        Self { inner: "sex".to_owned() }
    }
    /// ### Is Sexes array
    ///
    /// Returns true if value is sexes array
    pub fn is_sexes_array(&self) -> bool {
        self.inner.as_str() == "sexesArray"
    }
    /// ### Sexes array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn sexes_array() -> Self {
        Self { inner: "sexesArray".to_owned() }
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

impl From<SupportScalarFields> for Value {
    fn from(value: SupportScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for SupportScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => SupportScalarFields::bool(),
                "boolArray" => SupportScalarFields::bool_array(),
                "date" => SupportScalarFields::date(),
                "dateArray" => SupportScalarFields::date_array(),
                "dateTime" => SupportScalarFields::date_time(),
                "dateTimeArray" => SupportScalarFields::date_time_array(),
                "decimal" => SupportScalarFields::decimal(),
                "decimalArray" => SupportScalarFields::decimal_array(),
                "float32" => SupportScalarFields::float_32(),
                "float32Array" => SupportScalarFields::float_32_array(),
                "float64" => SupportScalarFields::float_64(),
                "float64Array" => SupportScalarFields::float_64_array(),
                "id" => SupportScalarFields::id(),
                "int32" => SupportScalarFields::int_32(),
                "int32Array" => SupportScalarFields::int_32_array(),
                "int64" => SupportScalarFields::int_64(),
                "int64Array" => SupportScalarFields::int_64_array(),
                "sex" => SupportScalarFields::sex(),
                "sexesArray" => SupportScalarFields::sexes_array(),
                "string" => SupportScalarFields::string(),
                "stringArray" => SupportScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to SupportScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &SupportScalarFields {

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
                "sex" => unsafe { &*(enum_variant as *const str as *const Self) },
                "sexesArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &SupportScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportScalarFields"))
        }
    }
}

impl AsInterface for SupportScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => SupportScalarFields::bool(),
                "boolArray" => SupportScalarFields::bool_array(),
                "date" => SupportScalarFields::date(),
                "dateArray" => SupportScalarFields::date_array(),
                "dateTime" => SupportScalarFields::date_time(),
                "dateTimeArray" => SupportScalarFields::date_time_array(),
                "decimal" => SupportScalarFields::decimal(),
                "decimalArray" => SupportScalarFields::decimal_array(),
                "float32" => SupportScalarFields::float_32(),
                "float32Array" => SupportScalarFields::float_32_array(),
                "float64" => SupportScalarFields::float_64(),
                "float64Array" => SupportScalarFields::float_64_array(),
                "id" => SupportScalarFields::id(),
                "int32" => SupportScalarFields::int_32(),
                "int32Array" => SupportScalarFields::int_32_array(),
                "int64" => SupportScalarFields::int_64(),
                "int64Array" => SupportScalarFields::int_64_array(),
                "sex" => SupportScalarFields::sex(),
                "sexesArray" => SupportScalarFields::sexes_array(),
                "string" => SupportScalarFields::string(),
                "stringArray" => SupportScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to SupportScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportScalarFields"))
        }
    }
}

impl AsInterfaceRef for SupportScalarFields {
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
                "sex" => unsafe { &*(enum_variant as *const str as *const Self) },
                "sexesArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &SupportScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportScalarFields"))
        }
    }
}
/// ## Support serializable scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct SupportSerializableScalarFields {
    inner: String,
}

impl SupportSerializableScalarFields {
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
    /// ### Is Sex
    ///
    /// Returns true if value is sex
    pub fn is_sex(&self) -> bool {
        self.inner.as_str() == "sex"
    }
    /// ### Sex
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn sex() -> Self {
        Self { inner: "sex".to_owned() }
    }
    /// ### Is Sexes array
    ///
    /// Returns true if value is sexes array
    pub fn is_sexes_array(&self) -> bool {
        self.inner.as_str() == "sexesArray"
    }
    /// ### Sexes array
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn sexes_array() -> Self {
        Self { inner: "sexesArray".to_owned() }
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

impl From<SupportSerializableScalarFields> for Value {
    fn from(value: SupportSerializableScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for SupportSerializableScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => SupportSerializableScalarFields::bool(),
                "boolArray" => SupportSerializableScalarFields::bool_array(),
                "date" => SupportSerializableScalarFields::date(),
                "dateArray" => SupportSerializableScalarFields::date_array(),
                "dateTime" => SupportSerializableScalarFields::date_time(),
                "dateTimeArray" => SupportSerializableScalarFields::date_time_array(),
                "decimal" => SupportSerializableScalarFields::decimal(),
                "decimalArray" => SupportSerializableScalarFields::decimal_array(),
                "float32" => SupportSerializableScalarFields::float_32(),
                "float32Array" => SupportSerializableScalarFields::float_32_array(),
                "float64" => SupportSerializableScalarFields::float_64(),
                "float64Array" => SupportSerializableScalarFields::float_64_array(),
                "id" => SupportSerializableScalarFields::id(),
                "int32" => SupportSerializableScalarFields::int_32(),
                "int32Array" => SupportSerializableScalarFields::int_32_array(),
                "int64" => SupportSerializableScalarFields::int_64(),
                "int64Array" => SupportSerializableScalarFields::int_64_array(),
                "sex" => SupportSerializableScalarFields::sex(),
                "sexesArray" => SupportSerializableScalarFields::sexes_array(),
                "string" => SupportSerializableScalarFields::string(),
                "stringArray" => SupportSerializableScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to SupportSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportSerializableScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &SupportSerializableScalarFields {

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
                "sex" => unsafe { &*(enum_variant as *const str as *const Self) },
                "sexesArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &SupportSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportSerializableScalarFields"))
        }
    }
}

impl AsInterface for SupportSerializableScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "bool" => SupportSerializableScalarFields::bool(),
                "boolArray" => SupportSerializableScalarFields::bool_array(),
                "date" => SupportSerializableScalarFields::date(),
                "dateArray" => SupportSerializableScalarFields::date_array(),
                "dateTime" => SupportSerializableScalarFields::date_time(),
                "dateTimeArray" => SupportSerializableScalarFields::date_time_array(),
                "decimal" => SupportSerializableScalarFields::decimal(),
                "decimalArray" => SupportSerializableScalarFields::decimal_array(),
                "float32" => SupportSerializableScalarFields::float_32(),
                "float32Array" => SupportSerializableScalarFields::float_32_array(),
                "float64" => SupportSerializableScalarFields::float_64(),
                "float64Array" => SupportSerializableScalarFields::float_64_array(),
                "id" => SupportSerializableScalarFields::id(),
                "int32" => SupportSerializableScalarFields::int_32(),
                "int32Array" => SupportSerializableScalarFields::int_32_array(),
                "int64" => SupportSerializableScalarFields::int_64(),
                "int64Array" => SupportSerializableScalarFields::int_64_array(),
                "sex" => SupportSerializableScalarFields::sex(),
                "sexesArray" => SupportSerializableScalarFields::sexes_array(),
                "string" => SupportSerializableScalarFields::string(),
                "stringArray" => SupportSerializableScalarFields::string_array(),
                _ => Err(Error::new("cannot convert value to SupportSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportSerializableScalarFields"))
        }
    }
}

impl AsInterfaceRef for SupportSerializableScalarFields {
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
                "sex" => unsafe { &*(enum_variant as *const str as *const Self) },
                "sexesArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                "string" => unsafe { &*(enum_variant as *const str as *const Self) },
                "stringArray" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &SupportSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportSerializableScalarFields"))
        }
    }
}
/// ## Support relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct SupportRelations {
    inner: String,
}

impl SupportRelations {
}

impl From<SupportRelations> for Value {
    fn from(value: SupportRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for SupportRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &SupportRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportRelations"))
        }
    }
}

impl AsInterface for SupportRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportRelations"))
        }
    }
}

impl AsInterfaceRef for SupportRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportRelations"))
        }
    }
}
/// ## Support direct relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct SupportDirectRelations {
    inner: String,
}

impl SupportDirectRelations {
}

impl From<SupportDirectRelations> for Value {
    fn from(value: SupportDirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for SupportDirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportDirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &SupportDirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportDirectRelations"))
        }
    }
}

impl AsInterface for SupportDirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportDirectRelations"))
        }
    }
}

impl AsInterfaceRef for SupportDirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportDirectRelations"))
        }
    }
}
/// ## Support indirect relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct SupportIndirectRelations {
    inner: String,
}

impl SupportIndirectRelations {
}

impl From<SupportIndirectRelations> for Value {
    fn from(value: SupportIndirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for SupportIndirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportIndirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &SupportIndirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportIndirectRelations"))
        }
    }
}

impl AsInterface for SupportIndirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to SupportIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to SupportIndirectRelations"))
        }
    }
}

impl AsInterfaceRef for SupportIndirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &SupportIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &SupportIndirectRelations"))
        }
    }
}

/// ## Support
///
/// This model doesn't have a description.
pub struct SupportModel {
    ctx: model::Ctx,
}

impl SupportModel {
    /// Find many support objects.
    pub async fn find_many_objects(&self, query: impl Borrow<Value>) -> Result<Vec<Support>> {
        Ok(self.ctx.find_many(query.borrow()).await?)
    }

    /// Find a unique support object.
    pub async fn find_unique_object(&self, query: impl Borrow<Value>) -> Result<Option<Support>> {
        Ok(self.ctx.find_unique(query.borrow()).await?)
    }

    /// Find a support object.
    pub async fn find_first_object(&self, query: impl Borrow<Value>) -> Result<Option<Support>> {
        Ok(self.ctx.find_first(query.borrow()).await?)
    }

    /// Create a new support object.
    pub async fn create_object(&self, values: impl Borrow<Value>) -> Result<Support> {
        Ok(self.ctx.create_object::<Support>(values.borrow()).await?)
    }

    /// Create an empty support object.
    pub async fn create_default_object(&self) -> Result<Support> {
        Ok(self.ctx.create_object::<Support>(teon!({}).borrow()).await?)
    }

    /// Count objects on support.
    pub async fn count_objects(&self, query: impl Borrow<Value>) -> Result<usize> {
        Ok(self.ctx.count_objects(query.borrow()).await?)
    }

    /// Count fields on support.
    pub async fn count_fields(&self, query: impl Borrow<Value>) -> Result<SupportCountAggregateResult> {
        Ok(SupportCountAggregateResult::from_value(self.ctx.count_fields(query.borrow()).await?)?)
    }

    /// Aggregate on support.
    pub async fn aggregate(&self, query: impl Borrow<Value>) -> Result<SupportAggregateResult> {
        Ok(SupportAggregateResult::from_value(self.ctx.aggregate(query.borrow()).await?)?)
    }

    /// Group by on support.
    pub async fn group_by(&self, query: impl Borrow<Value>) -> Result<Vec<SupportAggregateResult>> {
        let values: Vec<Value> = self.ctx.group_by(query.borrow()).await?;
        let mut result = vec![];
        for value in values.into_iter() {
            result.push(SupportAggregateResult::from_value(value)?);
        }
        Ok(result)
    }

    
    /// Run a custom SQL clause.
    pub async fn sql<T, E>(&self, sql: &str) -> Result<Vec<T>> where T: TryFrom<Value, Error=E>, Error: From<E> {
        self.ctx.sql(sql).await
    }
    
}

#[derive(Clone)]
pub struct Support {
    inner: model::Object,
}

impl Support {

    /// Whether this support is new.
    pub fn is_new(&self) -> bool {
        self.inner.is_new()
    }

    /// Whether this support is modified.
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }

    /// Set new values to a support. Validations and transformations are
    /// triggered.
    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.set_teon(values.as_ref()).await
    }

    /// Update with new values to a support. Validations and transformations are
    /// not triggered.
    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.update_teon(values.as_ref()).await
    }

    /// Save this support.
    pub async fn save(&self) -> Result<()> {
        self.inner.save().await
    }

    /// Delete this support.
    pub async fn delete(&self) -> Result<()> {
        self.inner.delete().await
    }

    /// Convert this support object to teon.
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
    /// ## Sex
    ///
    /// This field doesn't have a description.
    pub fn sex(&self) -> Result<Option<Sex>> {
        let value: Value = self.inner.get("sex").unwrap();
        Ok(match value {
            Value::Null => None,
            _ => Some(value.try_into()?),
        })
    }

    /// ## Sex
    ///
    /// This field doesn't have a description.
    pub fn set_sex(&self, new_value: Option<Sex>) -> Result<()> {
        self.inner.set("sex", match new_value {
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
    /// ## Sexes array
    ///
    /// This field doesn't have a description.
    pub fn sexes_array(&self) -> Result<Option<Vec<Sex>>> {
        self.inner.get("sexesArray")
    }

    /// ## Sexes array
    ///
    /// This field doesn't have a description.
    pub fn set_sexes_array(&self, new_value: Option<Vec<Sex>>) -> Result<()> {
        self.inner.set("sexesArray", new_value)
    }
}

impl From<Support> for model::Object {
    fn from(value: Support) -> Self {
        value.inner.clone()
    }
}

impl From<model::Object> for Support {
    fn from(value: model::Object) -> Self {
        Self { inner: value }
    }
}

impl From<Support> for Value {
    fn from(value: Support) -> Self {
        Value::ModelObject(value.inner.clone())
    }
}

impl AsInterface for Support {
    fn from_value(value: Value) -> Result<Self> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl TryFrom<Value> for Support {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl Debug for Support {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Support {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ExtractFromPipelineCtx for Support {
    fn extract(_: &Arguments, ctx: &pipeline::Ctx) -> Self {
        Support {
            inner: ctx.object().clone(),
        }
    }
}

pub trait SupportSelectTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportSelect {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSelect {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSelectTrait for SupportSelect { }

impl AsInterface for SupportSelect {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSelect> for Value {
    fn from(value: SupportSelect) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSelect {

    fn from_value_ref(value: &Value) -> Result<&SupportSelect> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSelect)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSelect {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSelect {
    fn extract(request: &'a Request) -> Self {
        SupportSelect::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportIncludeTrait: Interface {
}

#[repr(transparent)]
pub struct SupportInclude {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportInclude {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportIncludeTrait for SupportInclude { }

impl AsInterface for SupportInclude {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportInclude> for Value {
    fn from(value: SupportInclude) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportInclude {

    fn from_value_ref(value: &Value) -> Result<&SupportInclude> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportInclude)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportInclude {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportInclude {
    fn extract(request: &'a Request) -> Self {
        SupportInclude::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportWhereInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&SupportWhereInput>> {
        Some(Vec::<&SupportWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<SupportWhereInput>>) {
    
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
    fn not(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<SupportWhereInput>) {
    
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
    fn or(&self) -> Option<Vec<&SupportWhereInput>> {
        Some(Vec::<&SupportWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<SupportWhereInput>>) {
    
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportWhereInputTrait for SupportWhereInput { }

impl AsInterface for SupportWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportWhereInput> for Value {
    fn from(value: SupportWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportWhereInput {

    fn from_value_ref(value: &Value) -> Result<&SupportWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportWhereInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportWhereInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportWhereInput {
    fn extract(request: &'a Request) -> Self {
        SupportWhereInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportWhereUniqueInputTrait: Interface {
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
pub struct SupportWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportWhereUniqueInputTrait for SupportWhereUniqueInput { }

impl AsInterface for SupportWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportWhereUniqueInput> for Value {
    fn from(value: SupportWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&SupportWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        SupportWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportScalarWhereWithAggregatesInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&SupportWhereInput>> {
        Some(Vec::<&SupportWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<SupportWhereInput>>) {
    
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
    fn not(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<SupportWhereInput>) {
    
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
    fn or(&self) -> Option<Vec<&SupportWhereInput>> {
        Some(Vec::<&SupportWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<SupportWhereInput>>) {
    
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportScalarWhereWithAggregatesInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportScalarWhereWithAggregatesInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportScalarWhereWithAggregatesInputTrait for SupportScalarWhereWithAggregatesInput { }

impl AsInterface for SupportScalarWhereWithAggregatesInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportScalarWhereWithAggregatesInput> for Value {
    fn from(value: SupportScalarWhereWithAggregatesInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportScalarWhereWithAggregatesInput {

    fn from_value_ref(value: &Value) -> Result<&SupportScalarWhereWithAggregatesInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportScalarWhereWithAggregatesInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportScalarWhereWithAggregatesInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportScalarWhereWithAggregatesInput {
    fn extract(request: &'a Request) -> Self {
        SupportScalarWhereWithAggregatesInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportRelationFilterTrait: Interface {
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn is(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("is")?).unwrap())
    }
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn set_is(&mut self, new_value: Option<SupportWhereInput>) {
    
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
    fn is_not(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("isNot")?).unwrap())
    }
    /// ## Is Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_is_not(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportRelationFilterTrait for SupportRelationFilter { }

impl AsInterface for SupportRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportRelationFilter> for Value {
    fn from(value: SupportRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&SupportRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportRelationFilter)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportRelationFilter {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportRelationFilter {
    fn extract(request: &'a Request) -> Self {
        SupportRelationFilter::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportListRelationFilterTrait: Interface {
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn every(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("every")?).unwrap())
    }
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn set_every(&mut self, new_value: Option<SupportWhereInput>) {
    
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
    fn none(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("none")?).unwrap())
    }
    /// ## None
    ///
    /// This synthesized field doesn't have a description.
    fn set_none(&mut self, new_value: Option<SupportWhereInput>) {
    
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
    fn some(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("some")?).unwrap())
    }
    /// ## Some
    ///
    /// This synthesized field doesn't have a description.
    fn set_some(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportListRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportListRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportListRelationFilterTrait for SupportListRelationFilter { }

impl AsInterface for SupportListRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportListRelationFilter> for Value {
    fn from(value: SupportListRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportListRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&SupportListRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportListRelationFilter)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportListRelationFilter {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportListRelationFilter {
    fn extract(request: &'a Request) -> Self {
        SupportListRelationFilter::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportOrderByInputTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportOrderByInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportOrderByInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportOrderByInputTrait for SupportOrderByInput { }

impl AsInterface for SupportOrderByInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportOrderByInput> for Value {
    fn from(value: SupportOrderByInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportOrderByInput {

    fn from_value_ref(value: &Value) -> Result<&SupportOrderByInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportOrderByInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportOrderByInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportOrderByInput {
    fn extract(request: &'a Request) -> Self {
        SupportOrderByInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCountAggregateInputTypeTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportCountAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCountAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCountAggregateInputTypeTrait for SupportCountAggregateInputType { }

impl AsInterface for SupportCountAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCountAggregateInputType> for Value {
    fn from(value: SupportCountAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCountAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&SupportCountAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCountAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCountAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCountAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        SupportCountAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSumAggregateInputTypeTrait: Interface {
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
pub struct SupportSumAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSumAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSumAggregateInputTypeTrait for SupportSumAggregateInputType { }

impl AsInterface for SupportSumAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSumAggregateInputType> for Value {
    fn from(value: SupportSumAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSumAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&SupportSumAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSumAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSumAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSumAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        SupportSumAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportAvgAggregateInputTypeTrait: Interface {
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
pub struct SupportAvgAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportAvgAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportAvgAggregateInputTypeTrait for SupportAvgAggregateInputType { }

impl AsInterface for SupportAvgAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportAvgAggregateInputType> for Value {
    fn from(value: SupportAvgAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportAvgAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&SupportAvgAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportAvgAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportAvgAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportAvgAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        SupportAvgAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportMinAggregateInputTypeTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportMinAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportMinAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportMinAggregateInputTypeTrait for SupportMinAggregateInputType { }

impl AsInterface for SupportMinAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportMinAggregateInputType> for Value {
    fn from(value: SupportMinAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportMinAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&SupportMinAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportMinAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportMinAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportMinAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        SupportMinAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportMaxAggregateInputTypeTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportMaxAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportMaxAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportMaxAggregateInputTypeTrait for SupportMaxAggregateInputType { }

impl AsInterface for SupportMaxAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportMaxAggregateInputType> for Value {
    fn from(value: SupportMaxAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportMaxAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&SupportMaxAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportMaxAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportMaxAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportMaxAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        SupportMaxAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCreateInputTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCreateInputTrait for SupportCreateInput { }

impl AsInterface for SupportCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCreateInput> for Value {
    fn from(value: SupportCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCreateInput {

    fn from_value_ref(value: &Value) -> Result<&SupportCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCreateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCreateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCreateInput {
    fn extract(request: &'a Request) -> Self {
        SupportCreateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateInputTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateInputTrait for SupportUpdateInput { }

impl AsInterface for SupportUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateInput> for Value {
    fn from(value: SupportUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCreateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn connect_or_create(&self) -> Option<&SupportConnectOrCreateInput> {
        Some(SupportConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<SupportConnectOrCreateInput>) {
    
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
    fn create(&self) -> Option<&SupportCreateInput> {
        Some(SupportCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<SupportCreateInput>) {
    
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
pub struct SupportCreateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCreateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCreateNestedOneInputTrait for SupportCreateNestedOneInput { }

impl AsInterface for SupportCreateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCreateNestedOneInput> for Value {
    fn from(value: SupportCreateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCreateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&SupportCreateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCreateNestedOneInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCreateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCreateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        SupportCreateNestedOneInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCreateNestedManyInputTrait: Interface {
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
pub struct SupportCreateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCreateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCreateNestedManyInputTrait for SupportCreateNestedManyInput { }

impl AsInterface for SupportCreateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCreateNestedManyInput> for Value {
    fn from(value: SupportCreateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCreateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&SupportCreateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCreateNestedManyInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCreateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCreateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        SupportCreateNestedManyInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn connect_or_create(&self) -> Option<&SupportConnectOrCreateInput> {
        Some(SupportConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<SupportConnectOrCreateInput>) {
    
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
    fn create(&self) -> Option<&SupportCreateInput> {
        Some(SupportCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<SupportCreateInput>) {
    
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
    fn set(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("set")?).unwrap())
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set_set(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn update(&self) -> Option<&SupportUpdateInput> {
        Some(SupportUpdateInput::from_value_ref(self.inner().get("update")?).unwrap())
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: Option<SupportUpdateInput>) {
    
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
    fn upsert(&self) -> Option<&SupportUpsertWithWhereUniqueInput> {
        Some(SupportUpsertWithWhereUniqueInput::from_value_ref(self.inner().get("upsert")?).unwrap())
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn set_upsert(&mut self, new_value: Option<SupportUpsertWithWhereUniqueInput>) {
    
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
pub struct SupportUpdateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateNestedOneInputTrait for SupportUpdateNestedOneInput { }

impl AsInterface for SupportUpdateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateNestedOneInput> for Value {
    fn from(value: SupportUpdateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateNestedOneInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateNestedOneInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateNestedManyInputTrait: Interface {
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
pub struct SupportUpdateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateNestedManyInputTrait for SupportUpdateNestedManyInput { }

impl AsInterface for SupportUpdateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateNestedManyInput> for Value {
    fn from(value: SupportUpdateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateNestedManyInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateNestedManyInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportConnectOrCreateInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &SupportCreateInput {
        SupportCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: SupportCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportConnectOrCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportConnectOrCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportConnectOrCreateInputTrait for SupportConnectOrCreateInput { }

impl AsInterface for SupportConnectOrCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportConnectOrCreateInput> for Value {
    fn from(value: SupportConnectOrCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportConnectOrCreateInput {

    fn from_value_ref(value: &Value) -> Result<&SupportConnectOrCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportConnectOrCreateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportConnectOrCreateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportConnectOrCreateInput {
    fn extract(request: &'a Request) -> Self {
        SupportConnectOrCreateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateWithWhereUniqueInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpdateWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateWithWhereUniqueInputTrait for SupportUpdateWithWhereUniqueInput { }

impl AsInterface for SupportUpdateWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateWithWhereUniqueInput> for Value {
    fn from(value: SupportUpdateWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateWithWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateWithWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpsertWithWhereUniqueInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &SupportCreateInput {
        SupportCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: SupportCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpsertWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpsertWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpsertWithWhereUniqueInputTrait for SupportUpsertWithWhereUniqueInput { }

impl AsInterface for SupportUpsertWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpsertWithWhereUniqueInput> for Value {
    fn from(value: SupportUpsertWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpsertWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpsertWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpsertWithWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpsertWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpsertWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpsertWithWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateManyWithWhereInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereInput {
        SupportWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpdateManyWithWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateManyWithWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateManyWithWhereInputTrait for SupportUpdateManyWithWhereInput { }

impl AsInterface for SupportUpdateManyWithWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateManyWithWhereInput> for Value {
    fn from(value: SupportUpdateManyWithWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateManyWithWhereInput {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateManyWithWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateManyWithWhereInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateManyWithWhereInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateManyWithWhereInput {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateManyWithWhereInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportResultTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportResultTrait for SupportResult { }

impl AsInterface for SupportResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportResult> for Value {
    fn from(value: SupportResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportResult {

    fn from_value_ref(value: &Value) -> Result<&SupportResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportResult {
    fn extract(request: &'a Request) -> Self {
        SupportResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCountAggregateResultTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportCountAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCountAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCountAggregateResultTrait for SupportCountAggregateResult { }

impl AsInterface for SupportCountAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCountAggregateResult> for Value {
    fn from(value: SupportCountAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCountAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportCountAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCountAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCountAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCountAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportCountAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSumAggregateResultTrait: Interface {
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
pub struct SupportSumAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSumAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSumAggregateResultTrait for SupportSumAggregateResult { }

impl AsInterface for SupportSumAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSumAggregateResult> for Value {
    fn from(value: SupportSumAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSumAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportSumAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSumAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSumAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSumAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportSumAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportAvgAggregateResultTrait: Interface {
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
pub struct SupportAvgAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportAvgAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportAvgAggregateResultTrait for SupportAvgAggregateResult { }

impl AsInterface for SupportAvgAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportAvgAggregateResult> for Value {
    fn from(value: SupportAvgAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportAvgAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportAvgAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportAvgAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportAvgAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportAvgAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportAvgAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportMinAggregateResultTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportMinAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportMinAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportMinAggregateResultTrait for SupportMinAggregateResult { }

impl AsInterface for SupportMinAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportMinAggregateResult> for Value {
    fn from(value: SupportMinAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportMinAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportMinAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportMinAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportMinAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportMinAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportMinAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportMaxAggregateResultTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportMaxAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportMaxAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportMaxAggregateResultTrait for SupportMaxAggregateResult { }

impl AsInterface for SupportMaxAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportMaxAggregateResult> for Value {
    fn from(value: SupportMaxAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportMaxAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportMaxAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportMaxAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportMaxAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportMaxAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportMaxAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportAggregateResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&SupportAvgAggregateResult> {
        Some(SupportAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<SupportAvgAggregateResult>) {
    
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
    fn count(&self) -> Option<&SupportCountAggregateResult> {
        Some(SupportCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<SupportCountAggregateResult>) {
    
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
    fn max(&self) -> Option<&SupportMaxAggregateResult> {
        Some(SupportMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<SupportMaxAggregateResult>) {
    
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
    fn min(&self) -> Option<&SupportMinAggregateResult> {
        Some(SupportMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<SupportMinAggregateResult>) {
    
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
    fn sum(&self) -> Option<&SupportSumAggregateResult> {
        Some(SupportSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<SupportSumAggregateResult>) {
    
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
pub struct SupportAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportAggregateResultTrait for SupportAggregateResult { }

impl AsInterface for SupportAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportAggregateResult> for Value {
    fn from(value: SupportAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&SupportAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportAggregateResult {
    fn extract(request: &'a Request) -> Self {
        SupportAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportGroupByResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&SupportAvgAggregateResult> {
        Some(SupportAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<SupportAvgAggregateResult>) {
    
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
    fn count(&self) -> Option<&SupportCountAggregateResult> {
        Some(SupportCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<SupportCountAggregateResult>) {
    
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
    fn max(&self) -> Option<&SupportMaxAggregateResult> {
        Some(SupportMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<SupportMaxAggregateResult>) {
    
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
    fn min(&self) -> Option<&SupportMinAggregateResult> {
        Some(SupportMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<SupportMinAggregateResult>) {
    
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
    fn sum(&self) -> Option<&SupportSumAggregateResult> {
        Some(SupportSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<SupportSumAggregateResult>) {
    
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportGroupByResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportGroupByResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportGroupByResultTrait for SupportGroupByResult { }

impl AsInterface for SupportGroupByResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportGroupByResult> for Value {
    fn from(value: SupportGroupByResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportGroupByResult {

    fn from_value_ref(value: &Value) -> Result<&SupportGroupByResult> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportGroupByResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportGroupByResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportGroupByResult {
    fn extract(request: &'a Request) -> Self {
        SupportGroupByResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
pub struct SupportArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportArgsTrait for SupportArgs { }

impl AsInterface for SupportArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportArgs> for Value {
    fn from(value: SupportArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportArgs {
    fn extract(request: &'a Request) -> Self {
        SupportArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportFindManyArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn distinct(&self) -> Option<&SupportSerializableScalarFields> {
        Some(SupportSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<SupportSerializableScalarFields>) {
    
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
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportFindManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportFindManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportFindManyArgsTrait for SupportFindManyArgs { }

impl AsInterface for SupportFindManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportFindManyArgs> for Value {
    fn from(value: SupportFindManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportFindManyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportFindManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportFindManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportFindManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportFindManyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportFindManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportFindFirstArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn distinct(&self) -> Option<&SupportSerializableScalarFields> {
        Some(SupportSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<SupportSerializableScalarFields>) {
    
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
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportFindFirstArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportFindFirstArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportFindFirstArgsTrait for SupportFindFirstArgs { }

impl AsInterface for SupportFindFirstArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportFindFirstArgs> for Value {
    fn from(value: SupportFindFirstArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportFindFirstArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportFindFirstArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportFindFirstArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportFindFirstArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportFindFirstArgs {
    fn extract(request: &'a Request) -> Self {
        SupportFindFirstArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportFindUniqueArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportFindUniqueArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportFindUniqueArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportFindUniqueArgsTrait for SupportFindUniqueArgs { }

impl AsInterface for SupportFindUniqueArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportFindUniqueArgs> for Value {
    fn from(value: SupportFindUniqueArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportFindUniqueArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportFindUniqueArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportFindUniqueArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportFindUniqueArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportFindUniqueArgs {
    fn extract(request: &'a Request) -> Self {
        SupportFindUniqueArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCreateArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &SupportCreateInput {
        SupportCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: SupportCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
pub struct SupportCreateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCreateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCreateArgsTrait for SupportCreateArgs { }

impl AsInterface for SupportCreateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCreateArgs> for Value {
    fn from(value: SupportCreateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCreateArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportCreateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCreateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCreateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCreateArgs {
    fn extract(request: &'a Request) -> Self {
        SupportCreateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpdateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateArgsTrait for SupportUpdateArgs { }

impl AsInterface for SupportUpdateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateArgs> for Value {
    fn from(value: SupportUpdateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateArgs {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpsertArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &SupportCreateInput {
        SupportCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: SupportCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpsertArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpsertArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpsertArgsTrait for SupportUpsertArgs { }

impl AsInterface for SupportUpsertArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpsertArgs> for Value {
    fn from(value: SupportUpsertArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpsertArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportUpsertArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpsertArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpsertArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpsertArgs {
    fn extract(request: &'a Request) -> Self {
        SupportUpsertArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCopyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportCopyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCopyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCopyArgsTrait for SupportCopyArgs { }

impl AsInterface for SupportCopyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCopyArgs> for Value {
    fn from(value: SupportCopyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCopyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportCopyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCopyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCopyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCopyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportCopyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportDeleteArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> &SupportWhereUniqueInput {
        SupportWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportDeleteArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportDeleteArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportDeleteArgsTrait for SupportDeleteArgs { }

impl AsInterface for SupportDeleteArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportDeleteArgs> for Value {
    fn from(value: SupportDeleteArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportDeleteArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportDeleteArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportDeleteArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportDeleteArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportDeleteArgs {
    fn extract(request: &'a Request) -> Self {
        SupportDeleteArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCreateManyArgsTrait: Interface {
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
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
pub struct SupportCreateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCreateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCreateManyArgsTrait for SupportCreateManyArgs { }

impl AsInterface for SupportCreateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCreateManyArgs> for Value {
    fn from(value: SupportCreateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCreateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportCreateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCreateManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCreateManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCreateManyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportCreateManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportUpdateManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn update(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &SupportWhereInput {
        SupportWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportUpdateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportUpdateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportUpdateManyArgsTrait for SupportUpdateManyArgs { }

impl AsInterface for SupportUpdateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportUpdateManyArgs> for Value {
    fn from(value: SupportUpdateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportUpdateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportUpdateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportUpdateManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportUpdateManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportUpdateManyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportUpdateManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportDeleteManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> &SupportWhereInput {
        SupportWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportDeleteManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportDeleteManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportDeleteManyArgsTrait for SupportDeleteManyArgs { }

impl AsInterface for SupportDeleteManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportDeleteManyArgs> for Value {
    fn from(value: SupportDeleteManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportDeleteManyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportDeleteManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportDeleteManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportDeleteManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportDeleteManyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportDeleteManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCopyManyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &SupportUpdateInput {
        SupportUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: SupportUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
    fn r#where(&self) -> &SupportWhereInput {
        SupportWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: SupportWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct SupportCopyManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCopyManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCopyManyArgsTrait for SupportCopyManyArgs { }

impl AsInterface for SupportCopyManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCopyManyArgs> for Value {
    fn from(value: SupportCopyManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCopyManyArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportCopyManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCopyManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCopyManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCopyManyArgs {
    fn extract(request: &'a Request) -> Self {
        SupportCopyManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportCountArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn distinct(&self) -> Option<&SupportSerializableScalarFields> {
        Some(SupportSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<SupportSerializableScalarFields>) {
    
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
    fn select(&self) -> Option<&SupportCountAggregateInputType> {
        Some(SupportCountAggregateInputType::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportCountAggregateInputType>) {
    
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
    fn r#where(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportCountArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportCountArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportCountArgsTrait for SupportCountArgs { }

impl AsInterface for SupportCountArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportCountArgs> for Value {
    fn from(value: SupportCountArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportCountArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportCountArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportCountArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportCountArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportCountArgs {
    fn extract(request: &'a Request) -> Self {
        SupportCountArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportAggregateArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&SupportAvgAggregateInputType> {
        Some(SupportAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<SupportAvgAggregateInputType>) {
    
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
    fn count(&self) -> Option<&SupportCountAggregateInputType> {
        Some(SupportCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<SupportCountAggregateInputType>) {
    
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
    fn max(&self) -> Option<&SupportMaxAggregateInputType> {
        Some(SupportMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<SupportMaxAggregateInputType>) {
    
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
    fn min(&self) -> Option<&SupportMinAggregateInputType> {
        Some(SupportMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<SupportMinAggregateInputType>) {
    
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
    fn sum(&self) -> Option<&SupportSumAggregateInputType> {
        Some(SupportSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<SupportSumAggregateInputType>) {
    
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
    fn cursor(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn distinct(&self) -> Option<&SupportSerializableScalarFields> {
        Some(SupportSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<SupportSerializableScalarFields>) {
    
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
    fn r#where(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportAggregateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportAggregateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportAggregateArgsTrait for SupportAggregateArgs { }

impl AsInterface for SupportAggregateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportAggregateArgs> for Value {
    fn from(value: SupportAggregateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportAggregateArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportAggregateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportAggregateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportAggregateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportAggregateArgs {
    fn extract(request: &'a Request) -> Self {
        SupportAggregateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportGroupByArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&SupportAvgAggregateInputType> {
        Some(SupportAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<SupportAvgAggregateInputType>) {
    
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
    fn count(&self) -> Option<&SupportCountAggregateInputType> {
        Some(SupportCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<SupportCountAggregateInputType>) {
    
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
    fn max(&self) -> Option<&SupportMaxAggregateInputType> {
        Some(SupportMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<SupportMaxAggregateInputType>) {
    
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
    fn min(&self) -> Option<&SupportMinAggregateInputType> {
        Some(SupportMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<SupportMinAggregateInputType>) {
    
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
    fn sum(&self) -> Option<&SupportSumAggregateInputType> {
        Some(SupportSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<SupportSumAggregateInputType>) {
    
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
    fn cursor(&self) -> Option<&SupportWhereUniqueInput> {
        Some(SupportWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<SupportWhereUniqueInput>) {
    
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
    fn distinct(&self) -> Option<&SupportSerializableScalarFields> {
        Some(SupportSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<SupportSerializableScalarFields>) {
    
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
    fn having(&self) -> Option<&SupportScalarWhereWithAggregatesInput> {
        Some(SupportScalarWhereWithAggregatesInput::from_value_ref(self.inner().get("having")?).unwrap())
    }
    /// ## Having
    ///
    /// This synthesized field doesn't have a description.
    fn set_having(&mut self, new_value: Option<SupportScalarWhereWithAggregatesInput>) {
    
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
    fn r#where(&self) -> Option<&SupportWhereInput> {
        Some(SupportWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<SupportWhereInput>) {
    
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
pub struct SupportGroupByArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportGroupByArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportGroupByArgsTrait for SupportGroupByArgs { }

impl AsInterface for SupportGroupByArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportGroupByArgs> for Value {
    fn from(value: SupportGroupByArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportGroupByArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportGroupByArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportGroupByArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportGroupByArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportGroupByArgs {
    fn extract(request: &'a Request) -> Self {
        SupportGroupByArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportScalarUpdateInputTrait: Interface {
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
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn sex(&self) -> Option<&Sex> {
        Some(Sex::from_value_ref(self.inner().get("sex")?).unwrap())
    }
    /// ## Sex
    ///
    /// This synthesized field doesn't have a description.
    fn set_sex(&mut self, new_value: Option<Sex>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sex".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sex");
            },
        }
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn sexes_array(&self) -> Option<Vec<&Sex>> {
        Some(Vec::<&Sex>::from_value_ref_vec(self.inner().get("sexesArray")?).unwrap())
    }
    /// ## Sexes Array
    ///
    /// This synthesized field doesn't have a description.
    fn set_sexes_array(&mut self, new_value: Option<Vec<Sex>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("sexesArray".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("sexesArray");
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
pub struct SupportScalarUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportScalarUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportScalarUpdateInputTrait for SupportScalarUpdateInput { }

impl AsInterface for SupportScalarUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportScalarUpdateInput> for Value {
    fn from(value: SupportScalarUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportScalarUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&SupportScalarUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportScalarUpdateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportScalarUpdateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportScalarUpdateInput {
    fn extract(request: &'a Request) -> Self {
        SupportScalarUpdateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSignInCheckerIdsTrait: Interface {
}

#[repr(transparent)]
pub struct SupportSignInCheckerIds {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSignInCheckerIds {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSignInCheckerIdsTrait for SupportSignInCheckerIds { }

impl AsInterface for SupportSignInCheckerIds {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSignInCheckerIds> for Value {
    fn from(value: SupportSignInCheckerIds) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSignInCheckerIds {

    fn from_value_ref(value: &Value) -> Result<&SupportSignInCheckerIds> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSignInCheckerIds)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSignInCheckerIds {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSignInCheckerIds {
    fn extract(request: &'a Request) -> Self {
        SupportSignInCheckerIds::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSignInCheckerCompanionsTrait: Interface {
}

#[repr(transparent)]
pub struct SupportSignInCheckerCompanions {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSignInCheckerCompanions {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSignInCheckerCompanionsTrait for SupportSignInCheckerCompanions { }

impl AsInterface for SupportSignInCheckerCompanions {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSignInCheckerCompanions> for Value {
    fn from(value: SupportSignInCheckerCompanions) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSignInCheckerCompanions {

    fn from_value_ref(value: &Value) -> Result<&SupportSignInCheckerCompanions> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSignInCheckerCompanions)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSignInCheckerCompanions {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSignInCheckerCompanions {
    fn extract(request: &'a Request) -> Self {
        SupportSignInCheckerCompanions::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSignInInputTrait: Interface {
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn credentials(&self) -> &SupportSignInArgs {
        SupportSignInArgs::from_value_ref(self.inner().get("credentials").unwrap()).unwrap()
    }
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn set_credentials(&mut self, new_value: SupportSignInArgs) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("credentials".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&SupportInclude> {
        Some(SupportInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<SupportInclude>) {
    
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
    fn select(&self) -> Option<&SupportSelect> {
        Some(SupportSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<SupportSelect>) {
    
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
pub struct SupportSignInInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSignInInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSignInInputTrait for SupportSignInInput { }

impl AsInterface for SupportSignInInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSignInInput> for Value {
    fn from(value: SupportSignInInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSignInInput {

    fn from_value_ref(value: &Value) -> Result<&SupportSignInInput> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSignInInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSignInInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSignInInput {
    fn extract(request: &'a Request) -> Self {
        SupportSignInInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait SupportSignInArgsTrait: Interface {
}

#[repr(transparent)]
pub struct SupportSignInArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for SupportSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &SupportSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for SupportSignInArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl SupportSignInArgsTrait for SupportSignInArgs { }

impl AsInterface for SupportSignInArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<SupportSignInArgs> for Value {
    fn from(value: SupportSignInArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for SupportSignInArgs {

    fn from_value_ref(value: &Value) -> Result<&SupportSignInArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const SupportSignInArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for SupportSignInArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a SupportSignInArgs {
    fn extract(request: &'a Request) -> Self {
        SupportSignInArgs::from_value_ref(request.body_value().unwrap()).unwrap()
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
    
    pub fn support(&self) -> SupportModel {
        SupportModel { ctx: self.ctx.model_ctx_for_model_at_path(&vec!["Support".to_owned()]).unwrap() }
    }
}


impl ExtractFromTransactionCtx for Teo {
    fn extract(ctx: &transaction::Ctx) -> Self {
        Teo {
            ctx: ctx.clone(),
        }
    }
}

impl<'a> ExtractFromRequest<'a> for Teo {
    fn extract(request: &'a Request) -> Self {
        Teo {
            ctx: request.transaction_ctx().clone(),
        }
    }
}

impl ExtractFromPipelineCtx for Teo {
    fn extract(_: &Arguments, ctx: &pipeline::Ctx) -> Self {
        Teo {
            ctx: ctx.transaction_ctx().clone(),
        }
    }
}
