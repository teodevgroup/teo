use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::result::Result;
use crate::prelude::UserCtx;

pub trait AsyncCallbackWithUserCtx: Send + Sync {
    fn call(&self, user_ctx: UserCtx) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> AsyncCallbackWithUserCtx for F where
    F: Fn(UserCtx) -> Fut + Send + Sync,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, t: UserCtx) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(t))
    }
}