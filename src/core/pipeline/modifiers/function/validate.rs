use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::validity::Validity;
use crate::core::teon::Value;

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
pub struct ValidateModifier<T, O> {
    callback: Arc<dyn ValidateArgument<T, O>>
}

impl<T, O> Debug for ValidateModifier<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("ValidateModifier");
        result.finish()
    }
}

impl<T, O> ValidateModifier<T, O> {
    pub fn new<F>(f: F) -> ValidateModifier<T, O> where
        T: From<Value> + Send + Sync,
        O: Into<Validity> + Send + Sync,
        F: ValidateArgument<T, O> + 'static {
        return ValidateModifier {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync> Modifier for ValidateModifier<T, O> {

    fn name(&self) -> &'static str {
        "validate"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let cb = self.callback.clone();
        let value = cb.call((&ctx).value.clone().into()).await;
        let validity = value.into();
        if validity.is_valid() {
            ctx
        } else {
            ctx.invalid(validity.invalid_reason().unwrap())
        }
    }
}

unsafe impl<T, O> Send for ValidateModifier<T, O> {}
unsafe impl<T, O> Sync for ValidateModifier<T, O> {}
