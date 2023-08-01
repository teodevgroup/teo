use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::core::result::Result;
use crate::prelude::{Req, Res, UserCtx};

type Next = dyn Fn(Req, UserCtx) -> dyn Future<Output=Result<Res>>;

pub struct MiddlewareCtx {
    pub(crate) req: Req,
    pub user_ctx: UserCtx,
    pub next: Arc<Next>,
}

pub trait Middleware: Send + Sync {
    fn call(&self, ctx: MiddlewareCtx) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(MiddlewareCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, ctx: MiddlewareCtx) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}