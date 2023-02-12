use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

pub enum PerformResult {
    Result(Result<()>)
}

impl From<()> for PerformResult {
    fn from(_: ()) -> Self {
        PerformResult::Result(Ok(()))
    }
}

impl From<Result<()>> for PerformResult {
    fn from(result: Result<()>) -> Self {
        PerformResult::Result(result)
    }
}

pub trait PerformArgument<T: From<Value> + Send + Sync, O: Into<PerformResult>>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, O>;
}

impl<T, F, O, Fut> PerformArgument<T, O> for F where
T: From<Value> + Send + Sync,
F: Fn(T) -> Fut + Sync + Send,
O: Into<PerformResult> + Send + Sync,
Fut: Future<Output = O> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, O> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct PerformItem<T, O> {
    callback: Arc<dyn PerformArgument<T, O>>
}

impl<T, O> Debug for PerformItem<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("PerformItem");
        result.finish()
    }
}

impl<T, O> PerformItem<T, O> {
    pub fn new<F>(f: F) -> PerformItem<T, O> where
        T: From<Value> + Send + Sync,
        F: PerformArgument<T, O> + 'static,
        O: Into<PerformResult> + Send + Sync {
        return PerformItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync, O: Into<PerformResult> + Send + Sync> Item for PerformItem<T, O> {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let cb = self.callback.clone();
        let result = cb.call((&ctx).value.clone().into()).await.into();
        match result {
            PerformResult::Result(result) => match result {
                Ok(_) => Ok(ctx),
                Err(error) => Err(ctx.unwrap_custom_error(error)),
            }
        }
    }
}

unsafe impl<T, O> Send for PerformItem<T, O> {}
unsafe impl<T, O> Sync for PerformItem<T, O> {}
