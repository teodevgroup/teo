use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::result::Result;

pub trait AsyncCallbackWithoutArgs: Send + Sync {
    fn call(&self) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> AsyncCallbackWithoutArgs for F where
    F: Fn() -> Fut + Sync + Send,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self) -> BoxFuture<'static, Result<()>> {
        Box::pin(self())
    }
}