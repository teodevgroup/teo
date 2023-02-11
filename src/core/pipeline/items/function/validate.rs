use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity;
use crate::core::teon::Value;
use crate::core::result::Result;

pub trait ValidateArgument<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, O>;
}

impl<T, O, F, Fut> ValidateArgument<T, O> for F where
    T: From<Value> + Send + Sync,
    O: Into<Validity> + Send + Sync,
    F: Fn(T) -> Fut + Sync + Send,
    Fut: Future<Output = O> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, O> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct ValidateItem<T, O> {
    callback: Arc<dyn ValidateArgument<T, O>>
}

impl<T, O> Debug for ValidateItem<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("ValidateItem");
        result.finish()
    }
}

impl<T, O> ValidateItem<T, O> {
    pub fn new<F>(f: F) -> ValidateItem<T, O> where
        T: From<Value> + Send + Sync,
        O: Into<Validity> + Send + Sync,
        F: ValidateArgument<T, O> + 'static {
        return ValidateItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync> Item for ValidateItem<T, O> {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let cb = self.callback.clone();
        let value = cb.call((&ctx).value.clone().into()).await;
        let validity = value.into();
        if validity.is_valid() {
            Ok(ctx)
        } else {
            Err(ctx.internal_server_error(validity.invalid_reason().unwrap()))
        }
    }
}

unsafe impl<T, O> Send for ValidateItem<T, O> {}
unsafe impl<T, O> Sync for ValidateItem<T, O> {}
