use std::fmt::{Debug, Formatter};
use std::future::Future;

use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::result::Result;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

pub trait TransformArgument<T: From<Value> + Send + Sync>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, T>;
}

impl<T, F, Fut> TransformArgument<T> for F where
    T: From<Value> + Send + Sync,
    F: Fn(T) -> Fut + Sync + Send,
    Fut: Future<Output = T> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, T> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct TransformItem<T> {
    callback: Arc<dyn TransformArgument<T>>
}

impl<T> Debug for TransformItem<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("TransformItem");
        result.finish()
    }
}

impl<T> TransformItem<T> {
    pub fn new<F>(f: F) -> TransformItem<T> where
        T: From<Value> + Send + Sync,
        F: TransformArgument<T> + 'static {
        return TransformItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: Into<Value> + From<Value> + Send + Sync> Item for TransformItem<T> {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let cb = self.callback.clone();
        let value = cb.call((&ctx).value.clone().into()).await;
        Ok(ctx.with_value(value.into()))
    }
}

unsafe impl<T> Send for TransformItem<T> {}
unsafe impl<T> Sync for TransformItem<T> {}
