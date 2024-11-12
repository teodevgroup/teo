
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use chrono::NaiveDate;
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use super::super::helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};





pub trait TokenInfoTrait: Interface {
    /// ## Token
    ///
    /// This interface field doesn't have a description.
    fn token(&self) -> &String {
        String::from_value_ref(self.inner().get("token").unwrap()).unwrap()
    }
    /// ## Token
    ///
    /// This interface field doesn't have a description.
    fn set_token(&mut self, new_value: String) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("token".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct TokenInfo {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for TokenInfo {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &TokenInfo {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for TokenInfo {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl TokenInfoTrait for TokenInfo { }

impl AsInterface for TokenInfo {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<TokenInfo> for Value {
    fn from(value: TokenInfo) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for TokenInfo {

    fn from_value_ref(value: &Value) -> Result<&TokenInfo> {
        Ok(unsafe {
            &*(value as *const Value as *const TokenInfo)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for TokenInfo {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a TokenInfo {
    fn extract(request: &'a Request) -> Self {
        TokenInfo::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub struct IdentityNamespace {
    pub(crate) ctx: transaction::Ctx,
}

impl From<transaction::Ctx> for IdentityNamespace {
    fn from(value: transaction::Ctx) -> Self {
        Self { ctx: value }
    }
}

impl IdentityNamespace {

    
}

