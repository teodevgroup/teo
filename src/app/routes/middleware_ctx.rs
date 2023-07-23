use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::result::Result;

pub struct MiddlewareCtx {

}

pub trait Middleware: Send + Sync {
    fn call(&self, ctx: &mut MiddlewareCtx) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(&mut MiddlewareCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, ctx: &mut MiddlewareCtx) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}