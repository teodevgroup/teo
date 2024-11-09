
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use super::super::helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};





pub struct AdminNamespace {
    pub(crate) ctx: transaction::Ctx,
}

impl From<transaction::Ctx> for AdminNamespace {
    fn from(value: transaction::Ctx) -> Self {
        Self { ctx: value }
    }
}

impl AdminNamespace {

    
}

