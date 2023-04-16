use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::callbacks::types::transform::{TransformArgument, TransformResult};
use crate::core::result::Result;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::prelude::Error;

#[derive(Clone)]
pub struct TransformItem<T, R> {
    callback: Arc<dyn TransformArgument<T, R>>
}

impl<T, R> Debug for TransformItem<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("TransformItem");
        result.finish()
    }
}

impl<T, R> TransformItem<T, R> {
    pub fn new<F>(f: F) -> TransformItem<T, R> where
        T: From<Value> + Send + Sync + Into<Value>,
        R: Into<TransformResult<T>> + Send + Sync,
        F: TransformArgument<T, R> + 'static {
        return TransformItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: Into<Value> + From<Value> + Send + Sync, R: Into<TransformResult<T>> + Send + Sync> Item for TransformItem<T, R> {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let cb = self.callback.clone();
        let value = cb.call((&ctx).value.clone().into()).await;
        let function_result = value.into();
        match function_result {
            TransformResult::Value(value) => Ok(ctx.with_value(value.into())),
            TransformResult::Result(result) => match result {
                Ok(value) => Ok(ctx.with_value(value.into())),
                Err(error) => Err(ctx.unwrap_custom_error(error)),
            }
        }
    }
}

unsafe impl<T, R> Send for TransformItem<T, R> {}
unsafe impl<T, R> Sync for TransformItem<T, R> {}
