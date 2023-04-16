use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

pub enum CallbackResult {
    Result(Result<()>)
}

impl From<()> for CallbackResult {
    fn from(_: ()) -> Self {
        CallbackResult::Result(Ok(()))
    }
}

impl From<Result<()>> for CallbackResult {
    fn from(result: Result<()>) -> Self {
        CallbackResult::Result(result)
    }
}

pub trait CallbackArgument<T: From<Value> + Send + Sync, O: Into<CallbackResult>>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, O>;
}

impl<T, F, O, Fut> CallbackArgument<T, O> for F where
    T: From<Value> + Send + Sync,
    F: Fn(T) -> Fut + Sync + Send,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, O> {
        Box::pin(self(args))
    }
}