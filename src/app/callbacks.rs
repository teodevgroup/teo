use std::future::Future;
use futures_util::future::BoxFuture;
use teo_runtime::connection::transaction::{Ctx, ExtractFromTransactionCtx};
use teo_result::Result;

pub trait AsyncCallback: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> AsyncCallback for F where
    F: Fn(Ctx) -> Fut + Send + Sync,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub trait AsyncCallbackArgument<I>: Send + Sync {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<()>>;
}

impl<A0, F, Fut> AsyncCallbackArgument<(A0,)> for F where
    A0: ExtractFromTransactionCtx + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, ctx: Ctx) -> BoxFuture<'static, Result<()>> {
        let value: A0 = ExtractFromTransactionCtx::extract(&ctx);
        Box::pin(self(value))
    }
}