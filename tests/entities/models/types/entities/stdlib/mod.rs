
pub mod admin;

pub mod bcrypt;

pub mod identity;

use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use bson::oid::ObjectId;
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use super::helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};
pub use admin::AdminNamespace;

pub use bcrypt::BcryptNamespace;

pub use identity::IdentityNamespace;



/// ## Sort Order
///
/// Represents the sort order
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct Sort {
    inner: String,
}

impl Sort {
    /// ### Is Asc
    ///
    /// Returns true if value is asc
    pub fn is_asc(&self) -> bool {
        self.inner.as_str() == "asc"
    }
    /// ### Asc
    ///
    /// This enum member doesn't have a description.
    pub fn asc() -> Self {
        Self { inner: "asc".to_owned() }
    }
    /// ### Is Desc
    ///
    /// Returns true if value is desc
    pub fn is_desc(&self) -> bool {
        self.inner.as_str() == "desc"
    }
    /// ### Desc
    ///
    /// This enum member doesn't have a description.
    pub fn desc() -> Self {
        Self { inner: "desc".to_owned() }
    }
}

impl From<Sort> for Value {
    fn from(value: Sort) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for Sort {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "asc" => Sort::asc(),
                "desc" => Sort::desc(),
                _ => Err(Error::new("cannot convert value to Sort"))?
            })
        } else {
            Err(Error::new("cannot convert value to Sort"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &Sort {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "asc" => unsafe { &*(enum_variant as *const str as *const Self) },
                "desc" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Sort"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Sort"))
        }
    }
}

impl AsInterface for Sort {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "asc" => Sort::asc(),
                "desc" => Sort::desc(),
                _ => Err(Error::new("cannot convert value to Sort"))?
            })
        } else {
            Err(Error::new("cannot convert value to Sort"))
        }
    }
}

impl AsInterfaceRef for Sort {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "asc" => unsafe { &*(enum_variant as *const str as *const Self) },
                "desc" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &Sort"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &Sort"))
        }
    }
}
/// ## String Match Mode
///
/// Whether the string query is case sensitive or not
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct StringMatchMode {
    inner: String,
}

impl StringMatchMode {
    /// ### Is Default
    ///
    /// Returns true if value is default
    pub fn is_default(&self) -> bool {
        self.inner.as_str() == "default"
    }
    /// ### Default
    ///
    /// This enum member doesn't have a description.
    pub fn default() -> Self {
        Self { inner: "default".to_owned() }
    }
    /// ### Is Case insensitive
    ///
    /// Returns true if value is case insensitive
    pub fn is_case_insensitive(&self) -> bool {
        self.inner.as_str() == "caseInsensitive"
    }
    /// ### Case insensitive
    ///
    /// This enum member doesn't have a description.
    pub fn case_insensitive() -> Self {
        Self { inner: "caseInsensitive".to_owned() }
    }
}

impl From<StringMatchMode> for Value {
    fn from(value: StringMatchMode) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for StringMatchMode {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "default" => StringMatchMode::default(),
                "caseInsensitive" => StringMatchMode::case_insensitive(),
                _ => Err(Error::new("cannot convert value to StringMatchMode"))?
            })
        } else {
            Err(Error::new("cannot convert value to StringMatchMode"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &StringMatchMode {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "default" => unsafe { &*(enum_variant as *const str as *const Self) },
                "caseInsensitive" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &StringMatchMode"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &StringMatchMode"))
        }
    }
}

impl AsInterface for StringMatchMode {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "default" => StringMatchMode::default(),
                "caseInsensitive" => StringMatchMode::case_insensitive(),
                _ => Err(Error::new("cannot convert value to StringMatchMode"))?
            })
        } else {
            Err(Error::new("cannot convert value to StringMatchMode"))
        }
    }
}

impl AsInterfaceRef for StringMatchMode {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "default" => unsafe { &*(enum_variant as *const str as *const Self) },
                "caseInsensitive" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &StringMatchMode"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &StringMatchMode"))
        }
    }
}



pub trait EmptyTrait: Interface {
}

#[repr(transparent)]
pub struct Empty {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for Empty {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &Empty {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for Empty {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl EmptyTrait for Empty { }

impl AsInterface for Empty {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<Empty> for Value {
    fn from(value: Empty) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for Empty {

    fn from_value_ref(value: &Value) -> Result<&Empty> {
        Ok(unsafe {
            &*(value as *const Value as *const Empty)
        })
    }
}

impl ExtractFromRequest for Empty {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait DataTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Data
    ///
    /// This interface field doesn't have a description.
    fn data(&self) -> &T {
        T::from_value_ref(self.inner().get("data").unwrap()).unwrap()
    }
    /// ## Data
    ///
    /// This interface field doesn't have a description.
    fn set_data(&mut self, new_value: T) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("data".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> DataTrait<T> for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<Data<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: Data<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&Data<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const Data<T>)
        })
    }
}

impl<T> ExtractFromRequest for Data<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait DataMetaTrait<T, U>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Data
    ///
    /// This interface field doesn't have a description.
    fn data(&self) -> &T {
        T::from_value_ref(self.inner().get("data").unwrap()).unwrap()
    }
    /// ## Data
    ///
    /// This interface field doesn't have a description.
    fn set_data(&mut self, new_value: T) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("data".to_owned(), new_value.into()).unwrap();
    }
    /// ## Meta
    ///
    /// This interface field doesn't have a description.
    fn meta(&self) -> &U {
        U::from_value_ref(self.inner().get("meta").unwrap()).unwrap()
    }
    /// ## Meta
    ///
    /// This interface field doesn't have a description.
    fn set_meta(&mut self, new_value: U) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("meta".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<(T, U)>,
}

impl<T, U> Borrow<Value> for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T, U> Borrow<Value> for &DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T, U> Interface for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T, U> DataMetaTrait<T, U> for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T, U> AsInterface for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T, U> From<DataMeta<T, U>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: DataMeta<T, U>) -> Self {
        value.inner
    }
}

impl<T, U> AsInterfaceRef for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&DataMeta<T, U>> {
        Ok(unsafe {
            &*(value as *const Value as *const DataMeta<T, U>)
        })
    }
}

impl<T, U> ExtractFromRequest for DataMeta<T, U> where T: Into<Value> + AsInterface + AsInterfaceRef, U: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait PagingInfoTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> &i64 {
        i64::from_value_ref(self.inner().get("count").unwrap()).unwrap()
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: i64) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("count".to_owned(), new_value.into()).unwrap();
    }
    /// ## Number of pages
    ///
    /// This interface field doesn't have a description.
    fn number_of_pages(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("numberOfPages")?).unwrap())
    }
    /// ## Number of pages
    ///
    /// This interface field doesn't have a description.
    fn set_number_of_pages(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("numberOfPages".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("numberOfPages");
            },
        }
    }
}

#[repr(transparent)]
pub struct PagingInfo {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for PagingInfo {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &PagingInfo {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for PagingInfo {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl PagingInfoTrait for PagingInfo { }

impl AsInterface for PagingInfo {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<PagingInfo> for Value {
    fn from(value: PagingInfo) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for PagingInfo {

    fn from_value_ref(value: &Value) -> Result<&PagingInfo> {
        Ok(unsafe {
            &*(value as *const Value as *const PagingInfo)
        })
    }
}

impl ExtractFromRequest for PagingInfo {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ResponseErrorTrait: Interface {
    /// ## Type
    ///
    /// This interface field doesn't have a description.
    fn r#type(&self) -> &String {
        String::from_value_ref(self.inner().get("type").unwrap()).unwrap()
    }
    /// ## Type
    ///
    /// This interface field doesn't have a description.
    fn set_type(&mut self, new_value: String) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("type".to_owned(), new_value.into()).unwrap();
    }
    /// ## Message
    ///
    /// This interface field doesn't have a description.
    fn message(&self) -> &String {
        String::from_value_ref(self.inner().get("message").unwrap()).unwrap()
    }
    /// ## Message
    ///
    /// This interface field doesn't have a description.
    fn set_message(&mut self, new_value: String) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("message".to_owned(), new_value.into()).unwrap();
    }
    /// ## Fields
    ///
    /// This interface field doesn't have a description.
    fn fields(&self) -> &Value {
        Value::from_value_ref(self.inner().get("fields").unwrap()).unwrap()
    }
    /// ## Fields
    ///
    /// This interface field doesn't have a description.
    fn set_fields(&mut self, new_value: Value) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("fields".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct ResponseError {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for ResponseError {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &ResponseError {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for ResponseError {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl ResponseErrorTrait for ResponseError { }

impl AsInterface for ResponseError {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<ResponseError> for Value {
    fn from(value: ResponseError) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for ResponseError {

    fn from_value_ref(value: &Value) -> Result<&ResponseError> {
        Ok(unsafe {
            &*(value as *const Value as *const ResponseError)
        })
    }
}

impl ExtractFromRequest for ResponseError {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait BoolFilterTrait: Interface {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct BoolFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for BoolFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &BoolFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for BoolFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl BoolFilterTrait for BoolFilter { }

impl AsInterface for BoolFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<BoolFilter> for Value {
    fn from(value: BoolFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for BoolFilter {

    fn from_value_ref(value: &Value) -> Result<&BoolFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const BoolFilter)
        })
    }
}

impl ExtractFromRequest for BoolFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait BoolNullableFilterTrait: Interface {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct BoolNullableFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for BoolNullableFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &BoolNullableFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for BoolNullableFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl BoolNullableFilterTrait for BoolNullableFilter { }

impl AsInterface for BoolNullableFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<BoolNullableFilter> for Value {
    fn from(value: BoolNullableFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for BoolNullableFilter {

    fn from_value_ref(value: &Value) -> Result<&BoolNullableFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const BoolNullableFilter)
        })
    }
}

impl ExtractFromRequest for BoolNullableFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait FilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn lt(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("lt")?).unwrap())
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn set_lt(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lt");
            },
        }
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn lte(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("lte")?).unwrap())
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn set_lte(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lte");
            },
        }
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn gt(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("gt")?).unwrap())
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn set_gt(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gt");
            },
        }
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn gte(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("gte")?).unwrap())
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn set_gte(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gte");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> FilterTrait<T> for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<Filter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: Filter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&Filter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const Filter<T>)
        })
    }
}

impl<T> ExtractFromRequest for Filter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait NullableFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn lt(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("lt")?).unwrap())
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn set_lt(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lt");
            },
        }
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn lte(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("lte")?).unwrap())
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn set_lte(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lte");
            },
        }
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn gt(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("gt")?).unwrap())
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn set_gt(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gt");
            },
        }
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn gte(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("gte")?).unwrap())
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn set_gte(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gte");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> NullableFilterTrait<T> for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<NullableFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: NullableFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&NullableFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const NullableFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for NullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait StringFilterTrait: Interface {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&String>> {
        Some(Vec::<&String>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<String>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn lt(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("lt")?).unwrap())
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn set_lt(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lt");
            },
        }
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn lte(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("lte")?).unwrap())
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn set_lte(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lte");
            },
        }
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn gt(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("gt")?).unwrap())
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn set_gt(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gt");
            },
        }
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn gte(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("gte")?).unwrap())
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn set_gte(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gte");
            },
        }
    }
    /// ## Contains
    ///
    /// This interface field doesn't have a description.
    fn contains(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("contains")?).unwrap())
    }
    /// ## Contains
    ///
    /// This interface field doesn't have a description.
    fn set_contains(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("contains".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("contains");
            },
        }
    }
    /// ## Starts with
    ///
    /// This interface field doesn't have a description.
    fn starts_with(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("startsWith")?).unwrap())
    }
    /// ## Starts with
    ///
    /// This interface field doesn't have a description.
    fn set_starts_with(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("startsWith".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("startsWith");
            },
        }
    }
    /// ## Ends with
    ///
    /// This interface field doesn't have a description.
    fn ends_with(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("endsWith")?).unwrap())
    }
    /// ## Ends with
    ///
    /// This interface field doesn't have a description.
    fn set_ends_with(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("endsWith".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("endsWith");
            },
        }
    }
    /// ## Matches
    ///
    /// This interface field doesn't have a description.
    fn matches(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("matches")?).unwrap())
    }
    /// ## Matches
    ///
    /// This interface field doesn't have a description.
    fn set_matches(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("matches".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("matches");
            },
        }
    }
    /// ## Mode
    ///
    /// This interface field doesn't have a description.
    fn mode(&self) -> Option<&StringMatchMode> {
        Some(StringMatchMode::from_value_ref(self.inner().get("mode")?).unwrap())
    }
    /// ## Mode
    ///
    /// This interface field doesn't have a description.
    fn set_mode(&mut self, new_value: Option<StringMatchMode>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("mode".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("mode");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct StringFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for StringFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &StringFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for StringFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl StringFilterTrait for StringFilter { }

impl AsInterface for StringFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<StringFilter> for Value {
    fn from(value: StringFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for StringFilter {

    fn from_value_ref(value: &Value) -> Result<&StringFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const StringFilter)
        })
    }
}

impl ExtractFromRequest for StringFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait StringNullableFilterTrait: Interface {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn lt(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("lt")?).unwrap())
    }
    /// ## Lt
    ///
    /// This interface field doesn't have a description.
    fn set_lt(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lt");
            },
        }
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn lte(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("lte")?).unwrap())
    }
    /// ## Lte
    ///
    /// This interface field doesn't have a description.
    fn set_lte(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("lte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("lte");
            },
        }
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn gt(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("gt")?).unwrap())
    }
    /// ## Gt
    ///
    /// This interface field doesn't have a description.
    fn set_gt(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gt");
            },
        }
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn gte(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("gte")?).unwrap())
    }
    /// ## Gte
    ///
    /// This interface field doesn't have a description.
    fn set_gte(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("gte".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("gte");
            },
        }
    }
    /// ## Contains
    ///
    /// This interface field doesn't have a description.
    fn contains(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("contains")?).unwrap())
    }
    /// ## Contains
    ///
    /// This interface field doesn't have a description.
    fn set_contains(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("contains".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("contains");
            },
        }
    }
    /// ## Starts with
    ///
    /// This interface field doesn't have a description.
    fn starts_with(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("startsWith")?).unwrap())
    }
    /// ## Starts with
    ///
    /// This interface field doesn't have a description.
    fn set_starts_with(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("startsWith".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("startsWith");
            },
        }
    }
    /// ## Ends with
    ///
    /// This interface field doesn't have a description.
    fn ends_with(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("endsWith")?).unwrap())
    }
    /// ## Ends with
    ///
    /// This interface field doesn't have a description.
    fn set_ends_with(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("endsWith".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("endsWith");
            },
        }
    }
    /// ## Matches
    ///
    /// This interface field doesn't have a description.
    fn matches(&self) -> Option<&String> {
        Some(String::from_value_ref(self.inner().get("matches")?).unwrap())
    }
    /// ## Matches
    ///
    /// This interface field doesn't have a description.
    fn set_matches(&mut self, new_value: Option<String>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("matches".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("matches");
            },
        }
    }
    /// ## Mode
    ///
    /// This interface field doesn't have a description.
    fn mode(&self) -> Option<&StringMatchMode> {
        Some(StringMatchMode::from_value_ref(self.inner().get("mode")?).unwrap())
    }
    /// ## Mode
    ///
    /// This interface field doesn't have a description.
    fn set_mode(&mut self, new_value: Option<StringMatchMode>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("mode".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("mode");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct StringNullableFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for StringNullableFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &StringNullableFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for StringNullableFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl StringNullableFilterTrait for StringNullableFilter { }

impl AsInterface for StringNullableFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<StringNullableFilter> for Value {
    fn from(value: StringNullableFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for StringNullableFilter {

    fn from_value_ref(value: &Value) -> Result<&StringNullableFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const StringNullableFilter)
        })
    }
}

impl ExtractFromRequest for StringNullableFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait EnumFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> EnumFilterTrait<T> for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<EnumFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: EnumFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&EnumFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const EnumFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for EnumFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait EnumNullableFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn r#in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("in")?).unwrap())
    }
    /// ## In
    ///
    /// This interface field doesn't have a description.
    fn set_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("in".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("in");
            },
        }
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn not_in(&self) -> Option<Vec<&Value>> {
        Some(Vec::<&Value>::from_value_ref_vec(self.inner().get("notIn")?).unwrap())
    }
    /// ## Not in
    ///
    /// This interface field doesn't have a description.
    fn set_not_in(&mut self, new_value: Option<Vec<Value>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("notIn".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("notIn");
            },
        }
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn not(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("not")?).unwrap())
    }
    /// ## Not
    ///
    /// This interface field doesn't have a description.
    fn set_not(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("not".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("not");
            },
        }
    }
}

#[repr(transparent)]
pub struct EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> EnumNullableFilterTrait<T> for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<EnumNullableFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: EnumNullableFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&EnumNullableFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const EnumNullableFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for EnumNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ArrayFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## Has
    ///
    /// This interface field doesn't have a description.
    fn has(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("has")?).unwrap())
    }
    /// ## Has
    ///
    /// This interface field doesn't have a description.
    fn set_has(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("has".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("has");
            },
        }
    }
    /// ## Has some
    ///
    /// This interface field doesn't have a description.
    fn has_some(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("hasSome")?).unwrap())
    }
    /// ## Has some
    ///
    /// This interface field doesn't have a description.
    fn set_has_some(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("hasSome".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("hasSome");
            },
        }
    }
    /// ## Has every
    ///
    /// This interface field doesn't have a description.
    fn has_every(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("hasEvery")?).unwrap())
    }
    /// ## Has every
    ///
    /// This interface field doesn't have a description.
    fn set_has_every(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("hasEvery".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("hasEvery");
            },
        }
    }
    /// ## Is empty
    ///
    /// This interface field doesn't have a description.
    fn is_empty(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("isEmpty")?).unwrap())
    }
    /// ## Is empty
    ///
    /// This interface field doesn't have a description.
    fn set_is_empty(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("isEmpty".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("isEmpty");
            },
        }
    }
    /// ## Length
    ///
    /// This interface field doesn't have a description.
    fn length(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("length")?).unwrap())
    }
    /// ## Length
    ///
    /// This interface field doesn't have a description.
    fn set_length(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("length".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("length");
            },
        }
    }
}

#[repr(transparent)]
pub struct ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> ArrayFilterTrait<T> for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<ArrayFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: ArrayFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&ArrayFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const ArrayFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for ArrayFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ArrayNullableFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn equals(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("equals")?).unwrap())
    }
    /// ## Equals
    ///
    /// This interface field doesn't have a description.
    fn set_equals(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("equals".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("equals");
            },
        }
    }
    /// ## Has
    ///
    /// This interface field doesn't have a description.
    fn has(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("has")?).unwrap())
    }
    /// ## Has
    ///
    /// This interface field doesn't have a description.
    fn set_has(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("has".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("has");
            },
        }
    }
    /// ## Has some
    ///
    /// This interface field doesn't have a description.
    fn has_some(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("hasSome")?).unwrap())
    }
    /// ## Has some
    ///
    /// This interface field doesn't have a description.
    fn set_has_some(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("hasSome".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("hasSome");
            },
        }
    }
    /// ## Has every
    ///
    /// This interface field doesn't have a description.
    fn has_every(&self) -> Option<Vec<&T>> {
        Some(Vec::<&T>::from_value_ref_vec(self.inner().get("hasEvery")?).unwrap())
    }
    /// ## Has every
    ///
    /// This interface field doesn't have a description.
    fn set_has_every(&mut self, new_value: Option<Vec<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("hasEvery".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("hasEvery");
            },
        }
    }
    /// ## Is empty
    ///
    /// This interface field doesn't have a description.
    fn is_empty(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("isEmpty")?).unwrap())
    }
    /// ## Is empty
    ///
    /// This interface field doesn't have a description.
    fn set_is_empty(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("isEmpty".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("isEmpty");
            },
        }
    }
    /// ## Length
    ///
    /// This interface field doesn't have a description.
    fn length(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("length")?).unwrap())
    }
    /// ## Length
    ///
    /// This interface field doesn't have a description.
    fn set_length(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("length".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("length");
            },
        }
    }
}

#[repr(transparent)]
pub struct ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> ArrayNullableFilterTrait<T> for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<ArrayNullableFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: ArrayNullableFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&ArrayNullableFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const ArrayNullableFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for ArrayNullableFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait BoolWithAggregatesFilterTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&BoolFilter> {
        Some(BoolFilter::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<BoolFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&BoolFilter> {
        Some(BoolFilter::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<BoolFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct BoolWithAggregatesFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for BoolWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &BoolWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for BoolWithAggregatesFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl BoolWithAggregatesFilterTrait for BoolWithAggregatesFilter { }

impl BoolFilterTrait for BoolWithAggregatesFilter { }

impl AsInterface for BoolWithAggregatesFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<BoolWithAggregatesFilter> for Value {
    fn from(value: BoolWithAggregatesFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for BoolWithAggregatesFilter {

    fn from_value_ref(value: &Value) -> Result<&BoolWithAggregatesFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const BoolWithAggregatesFilter)
        })
    }
}

impl ExtractFromRequest for BoolWithAggregatesFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait BoolNullableWithAggregatesFilterTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&BoolNullableFilter> {
        Some(BoolNullableFilter::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<BoolNullableFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&BoolNullableFilter> {
        Some(BoolNullableFilter::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<BoolNullableFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct BoolNullableWithAggregatesFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for BoolNullableWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &BoolNullableWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for BoolNullableWithAggregatesFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl BoolNullableWithAggregatesFilterTrait for BoolNullableWithAggregatesFilter { }

impl BoolNullableFilterTrait for BoolNullableWithAggregatesFilter { }

impl AsInterface for BoolNullableWithAggregatesFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<BoolNullableWithAggregatesFilter> for Value {
    fn from(value: BoolNullableWithAggregatesFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for BoolNullableWithAggregatesFilter {

    fn from_value_ref(value: &Value) -> Result<&BoolNullableWithAggregatesFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const BoolNullableWithAggregatesFilter)
        })
    }
}

impl ExtractFromRequest for BoolNullableWithAggregatesFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait IntNumberWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&Filter<f64>> {
        Some(Filter::<f64>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<Filter<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&Filter<i64>> {
        Some(Filter::<i64>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<Filter<i64>>) {
    
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
pub struct IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> IntNumberWithAggregatesFilterTrait<T> for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> FilterTrait<T> for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<IntNumberWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: IntNumberWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&IntNumberWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const IntNumberWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for IntNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait IntNumberNullableWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&NullableFilter<f64>> {
        Some(NullableFilter::<f64>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<NullableFilter<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&NullableFilter<i64>> {
        Some(NullableFilter::<i64>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<NullableFilter<i64>>) {
    
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
pub struct IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> IntNumberNullableWithAggregatesFilterTrait<T> for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> NullableFilterTrait<T> for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<IntNumberNullableWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: IntNumberNullableWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&IntNumberNullableWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const IntNumberNullableWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for IntNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait FloatNumberWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&Filter<f64>> {
        Some(Filter::<f64>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<Filter<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&Filter<f64>> {
        Some(Filter::<f64>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<Filter<f64>>) {
    
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
pub struct FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> FloatNumberWithAggregatesFilterTrait<T> for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> FilterTrait<T> for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<FloatNumberWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: FloatNumberWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&FloatNumberWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const FloatNumberWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for FloatNumberWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait FloatNumberNullableWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&NullableFilter<f64>> {
        Some(NullableFilter::<f64>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<NullableFilter<f64>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&NullableFilter<f64>> {
        Some(NullableFilter::<f64>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<NullableFilter<f64>>) {
    
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
pub struct FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> FloatNumberNullableWithAggregatesFilterTrait<T> for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> NullableFilterTrait<T> for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<FloatNumberNullableWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: FloatNumberNullableWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&FloatNumberNullableWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const FloatNumberNullableWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for FloatNumberNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait DecimalWithAggregatesFilterTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&Filter<BigDecimal>> {
        Some(Filter::<BigDecimal>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<Filter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&Filter<BigDecimal>> {
        Some(Filter::<BigDecimal>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<Filter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&Filter<BigDecimal>> {
        Some(Filter::<BigDecimal>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<Filter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&Filter<BigDecimal>> {
        Some(Filter::<BigDecimal>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<Filter<BigDecimal>>) {
    
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
pub struct DecimalWithAggregatesFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for DecimalWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &DecimalWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for DecimalWithAggregatesFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl DecimalWithAggregatesFilterTrait for DecimalWithAggregatesFilter { }

impl FilterTrait<BigDecimal> for DecimalWithAggregatesFilter { }

impl AsInterface for DecimalWithAggregatesFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<DecimalWithAggregatesFilter> for Value {
    fn from(value: DecimalWithAggregatesFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for DecimalWithAggregatesFilter {

    fn from_value_ref(value: &Value) -> Result<&DecimalWithAggregatesFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const DecimalWithAggregatesFilter)
        })
    }
}

impl ExtractFromRequest for DecimalWithAggregatesFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait DecimalNullableWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&NullableFilter<BigDecimal>> {
        Some(NullableFilter::<BigDecimal>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<NullableFilter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&NullableFilter<BigDecimal>> {
        Some(NullableFilter::<BigDecimal>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<NullableFilter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn avg(&self) -> Option<&NullableFilter<BigDecimal>> {
        Some(NullableFilter::<BigDecimal>::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This interface field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<NullableFilter<BigDecimal>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn sum(&self) -> Option<&NullableFilter<BigDecimal>> {
        Some(NullableFilter::<BigDecimal>::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This interface field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<NullableFilter<BigDecimal>>) {
    
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
pub struct DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> DecimalNullableWithAggregatesFilterTrait<T> for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> NullableFilterTrait<T> for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<DecimalNullableWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: DecimalNullableWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&DecimalNullableWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const DecimalNullableWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for DecimalNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait AggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&Filter<T>> {
        Some(Filter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<Filter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> AggregatesFilterTrait<T> for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> FilterTrait<T> for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<AggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: AggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&AggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const AggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for AggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait NullableAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&NullableFilter<T>> {
        Some(NullableFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<NullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> NullableAggregatesFilterTrait<T> for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> NullableFilterTrait<T> for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<NullableAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: NullableAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&NullableAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const NullableAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for NullableAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait StringWithAggregatesFilterTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&StringFilter> {
        Some(StringFilter::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<StringFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&StringFilter> {
        Some(StringFilter::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<StringFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct StringWithAggregatesFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for StringWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &StringWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for StringWithAggregatesFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl StringWithAggregatesFilterTrait for StringWithAggregatesFilter { }

impl StringFilterTrait for StringWithAggregatesFilter { }

impl AsInterface for StringWithAggregatesFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<StringWithAggregatesFilter> for Value {
    fn from(value: StringWithAggregatesFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for StringWithAggregatesFilter {

    fn from_value_ref(value: &Value) -> Result<&StringWithAggregatesFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const StringWithAggregatesFilter)
        })
    }
}

impl ExtractFromRequest for StringWithAggregatesFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait StringNullableWithAggregatesFilterTrait: Interface {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&StringNullableFilter> {
        Some(StringNullableFilter::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<StringNullableFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&StringNullableFilter> {
        Some(StringNullableFilter::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<StringNullableFilter>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct StringNullableWithAggregatesFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for StringNullableWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &StringNullableWithAggregatesFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for StringNullableWithAggregatesFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl StringNullableWithAggregatesFilterTrait for StringNullableWithAggregatesFilter { }

impl StringNullableFilterTrait for StringNullableWithAggregatesFilter { }

impl AsInterface for StringNullableWithAggregatesFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<StringNullableWithAggregatesFilter> for Value {
    fn from(value: StringNullableWithAggregatesFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for StringNullableWithAggregatesFilter {

    fn from_value_ref(value: &Value) -> Result<&StringNullableWithAggregatesFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const StringNullableWithAggregatesFilter)
        })
    }
}

impl ExtractFromRequest for StringNullableWithAggregatesFilter {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait EnumWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&EnumFilter<T>> {
        Some(EnumFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<EnumFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&EnumFilter<T>> {
        Some(EnumFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<EnumFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> EnumWithAggregatesFilterTrait<T> for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> EnumFilterTrait<T> for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<EnumWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: EnumWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&EnumWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const EnumWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for EnumWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait EnumNullableWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&EnumNullableFilter<T>> {
        Some(EnumNullableFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<EnumNullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&EnumNullableFilter<T>> {
        Some(EnumNullableFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<EnumNullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> EnumNullableWithAggregatesFilterTrait<T> for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> EnumNullableFilterTrait<T> for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<EnumNullableWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: EnumNullableWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&EnumNullableWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const EnumNullableWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for EnumNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ArrayWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&ArrayFilter<T>> {
        Some(ArrayFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ArrayFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&ArrayFilter<T>> {
        Some(ArrayFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ArrayFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> ArrayWithAggregatesFilterTrait<T> for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> ArrayFilterTrait<T> for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<ArrayWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: ArrayWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&ArrayWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const ArrayWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for ArrayWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ArrayNullableWithAggregatesFilterTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn count(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This interface field doesn't have a description.
    fn set_count(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn min(&self) -> Option<&ArrayNullableFilter<T>> {
        Some(ArrayNullableFilter::<T>::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This interface field doesn't have a description.
    fn set_min(&mut self, new_value: Option<ArrayNullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn max(&self) -> Option<&ArrayNullableFilter<T>> {
        Some(ArrayNullableFilter::<T>::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This interface field doesn't have a description.
    fn set_max(&mut self, new_value: Option<ArrayNullableFilter<T>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
}

#[repr(transparent)]
pub struct ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> ArrayNullableWithAggregatesFilterTrait<T> for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> ArrayNullableFilterTrait<T> for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<ArrayNullableWithAggregatesFilter<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: ArrayNullableWithAggregatesFilter<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&ArrayNullableWithAggregatesFilter<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const ArrayNullableWithAggregatesFilter<T>)
        })
    }
}

impl<T> ExtractFromRequest for ArrayNullableWithAggregatesFilter<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait NumberAtomicUpdateOperationInputTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Increment
    ///
    /// This interface field doesn't have a description.
    fn increment(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("increment")?).unwrap())
    }
    /// ## Increment
    ///
    /// This interface field doesn't have a description.
    fn set_increment(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("increment".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("increment");
            },
        }
    }
    /// ## Decrement
    ///
    /// This interface field doesn't have a description.
    fn decrement(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("decrement")?).unwrap())
    }
    /// ## Decrement
    ///
    /// This interface field doesn't have a description.
    fn set_decrement(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("decrement".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("decrement");
            },
        }
    }
    /// ## Multiply
    ///
    /// This interface field doesn't have a description.
    fn multiply(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("multiply")?).unwrap())
    }
    /// ## Multiply
    ///
    /// This interface field doesn't have a description.
    fn set_multiply(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("multiply".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("multiply");
            },
        }
    }
    /// ## Divide
    ///
    /// This interface field doesn't have a description.
    fn divide(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("divide")?).unwrap())
    }
    /// ## Divide
    ///
    /// This interface field doesn't have a description.
    fn set_divide(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("divide".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("divide");
            },
        }
    }
}

#[repr(transparent)]
pub struct NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> NumberAtomicUpdateOperationInputTrait<T> for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<NumberAtomicUpdateOperationInput<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: NumberAtomicUpdateOperationInput<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&NumberAtomicUpdateOperationInput<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const NumberAtomicUpdateOperationInput<T>)
        })
    }
}

impl<T> ExtractFromRequest for NumberAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

pub trait ArrayAtomicUpdateOperationInputTrait<T>: Interface where T: Into<Value> + AsInterface + AsInterfaceRef {
    /// ## Push
    ///
    /// This interface field doesn't have a description.
    fn push(&self) -> Option<&T> {
        Some(T::from_value_ref(self.inner().get("push")?).unwrap())
    }
    /// ## Push
    ///
    /// This interface field doesn't have a description.
    fn set_push(&mut self, new_value: Option<T>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("push".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("push");
            },
        }
    }
}

#[repr(transparent)]
pub struct ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    inner: Value,
    phantom_data: PhantomData<T>,
}

impl<T> Borrow<Value> for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Borrow<Value> for &ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl<T> Interface for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl<T> ArrayAtomicUpdateOperationInputTrait<T> for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef { }

impl<T> AsInterface for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl<T> From<ArrayAtomicUpdateOperationInput<T>> for Value where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn from(value: ArrayAtomicUpdateOperationInput<T>) -> Self {
        value.inner
    }
}

impl<T> AsInterfaceRef for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {

    fn from_value_ref(value: &Value) -> Result<&ArrayAtomicUpdateOperationInput<T>> {
        Ok(unsafe {
            &*(value as *const Value as *const ArrayAtomicUpdateOperationInput<T>)
        })
    }
}

impl<T> ExtractFromRequest for ArrayAtomicUpdateOperationInput<T> where T: Into<Value> + AsInterface + AsInterfaceRef {
    fn extract(request: &Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

