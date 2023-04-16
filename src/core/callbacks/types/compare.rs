use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::callbacks::types::validate::ValidateResult;
use crate::core::teon::Value;

pub trait CompareArgument<T: From<Value> + Send + Sync, O: Into<ValidateResult> + Send + Sync>: Send + Sync {
    fn call(&self, old: T, new: T) -> BoxFuture<'static, O>;
}

impl<T, O, F, Fut> CompareArgument<T, O> for F where
    T: From<Value> + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(T, T) -> Fut + Sync + Send,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, old: T, new: T) -> BoxFuture<'static, O> {
        Box::pin(self(old, new))
    }
}