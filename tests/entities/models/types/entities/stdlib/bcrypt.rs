#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use super::super::helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};





pub struct BcryptNamespace {
    pub(crate) ctx: transaction::Ctx,
}

impl From<transaction::Ctx> for BcryptNamespace {
    fn from(value: transaction::Ctx) -> Self {
        Self { ctx: value }
    }
}

impl BcryptNamespace {

    
}

