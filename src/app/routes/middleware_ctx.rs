use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::result::Result;
use crate::prelude::Res;
use crate::server::ReqCtx;

pub trait Next {
    fn call(&self, req_ctx: ReqCtx) -> BoxFuture<'static, Result<Res>>;
}

pub trait Middleware: Send + Sync {
    fn call(&self, req_ctx: ReqCtx, next: Box<dyn Next>) -> BoxFuture<'static, Result<Res>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(ReqCtx, Box<dyn Next>) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, req_ctx: ReqCtx, next: Box<dyn Next>) -> BoxFuture<'static, Result<Res>> {
        Box::pin(self(req_ctx, next))
    }
}
