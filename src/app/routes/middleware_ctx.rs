use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use indexmap::IndexMap;
use crate::core::result::Result;
use crate::prelude::Res;
use crate::server::ReqCtx;

pub trait Next: Send + Sync {
    fn call(&self, req_ctx: ReqCtx) -> BoxFuture<'static, Result<Res>>;
}

impl<F, Fut> Next for F where
    F: Fn(ReqCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, req_ctx: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        Box::pin(self(req_ctx))
    }
}

pub trait Middleware: Send + Sync {
    fn call(&self, req_ctx: ReqCtx, next: &'static dyn Next) -> BoxFuture<'static, Result<Res>>;
}

impl<F, Fut> Middleware for F where
    F: Fn(ReqCtx, &'static dyn Next) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, req_ctx: ReqCtx, next: &'static dyn Next) -> BoxFuture<'static, Result<Res>> {
        Box::pin(self(req_ctx, next))
    }
}

pub(crate) fn combine_middleware(middlewares: IndexMap<&'static str, &'static dyn Middleware>) -> &'static dyn Middleware {
    match middlewares.len() {
        0 => Box::leak(Box::new(|req_ctx: ReqCtx, next: &'static dyn Next| async move {
            next.call(req_ctx).await
        })),
        1 => middlewares.first().unwrap().1.clone(),
        2 => join_middleware(*middlewares.get_index(1).unwrap().1, *middlewares.get_index(0).unwrap().1),
        _ => {
            let inner_most = *middlewares.first().unwrap().1;
            let mut result = join_middleware(*middlewares.get_index(1).unwrap().1, inner_most);
            for (index, (_, middleware)) in middlewares.iter().enumerate() {
                if index >= 2 {
                    result = join_middleware(*middleware, result);
                }
            }
            result
        }
    }
}

fn join_middleware(outer: &'static dyn Middleware, inner: &'static dyn Middleware) -> &'static dyn Middleware {
    return Box::leak(Box::new(move |req_ctx: ReqCtx, next: &'static dyn Next| async move {
        outer.call(req_ctx, Box::leak(Box::new(move |req_ctx: ReqCtx| async move {
            inner.call(req_ctx, next).await
        }))).await
    }))
}