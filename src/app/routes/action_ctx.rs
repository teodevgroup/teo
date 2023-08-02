use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::app::routes::req::Req;
use crate::app::routes::req_local::ReqLocal;
use crate::app::routes::res::Res;
use crate::core::result::Result;
use crate::server::ReqCtx;

pub trait ActionCtxArgument<A>: Send + Sync + 'static {
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>>;
}

pub trait ExtractValueFromReqCtx {
    fn extract(ctx_base: &ReqCtx) -> Self;
}

impl<A0, F, Fut> ActionCtxArgument<(A0,)> for F where
    A0: ExtractValueFromReqCtx + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        let value: A0 = ExtractValueFromReqCtx::extract(&ctx_base);
        Box::pin(self(value))
    }
}

impl<A0, A1, F, Fut> ActionCtxArgument<(A0, A1)> for F where
    A0: ExtractValueFromReqCtx + Send + Sync,
    A1: ExtractValueFromReqCtx + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        let value0: A0 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value1: A1 = ExtractValueFromReqCtx::extract(&ctx_base);
        Box::pin(self(value0, value1))
    }
}

impl<A0, A1, A2, F, Fut> ActionCtxArgument<(A0, A1, A2)> for F where
    A0: ExtractValueFromReqCtx + Send + Sync,
    A1: ExtractValueFromReqCtx + Send + Sync,
    A2: ExtractValueFromReqCtx + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        let value0: A0 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value1: A1 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value2: A2 = ExtractValueFromReqCtx::extract(&ctx_base);
        Box::pin(self(value0, value1, value2))
    }
}

impl<A0, A1, A2, A3, F, Fut> ActionCtxArgument<(A0, A1, A2, A3)> for F where
    A0: ExtractValueFromReqCtx + Send + Sync,
    A1: ExtractValueFromReqCtx + Send + Sync,
    A2: ExtractValueFromReqCtx + Send + Sync,
    A3: ExtractValueFromReqCtx + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + Clone + 'static,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        let value0: A0 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value1: A1 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value2: A2 = ExtractValueFromReqCtx::extract(&ctx_base);
        let value3: A3 = ExtractValueFromReqCtx::extract(&ctx_base);
        Box::pin(self(value0, value1, value2, value3))
    }
}

pub(crate) trait ActionHandlerDefTrait: Send + Sync {
    fn group(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>>;
}

pub(crate) struct ActionHandlerDef<A> {
    pub(crate) group: &'static str,
    pub(crate) name: &'static str,
    pub(crate) f: Arc<dyn ActionCtxArgument<A>>,
}

impl<A: 'static> ActionHandlerDefTrait for ActionHandlerDef<A> {
    fn group(&self) -> &'static str {
        self.group
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn call(&self, ctx_base: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        self.f.call(ctx_base)
    }
}

pub trait ActionHandler: Send + Sync {
    fn call(&self, ctx: ReqCtx) -> BoxFuture<'static, Result<Res>>;
}

impl<F, Fut> ActionHandler for F where
    F: Fn(ReqCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx: ReqCtx) -> BoxFuture<'static, Result<Res>> {
        Box::pin(self(ctx))
    }
}

impl ExtractValueFromReqCtx for Req {
    fn extract(ctx_base: &ReqCtx) -> Self {
        ctx_base.req.clone()
    }
}

impl ExtractValueFromReqCtx for ReqLocal {
    fn extract(req_ctx: &ReqCtx) -> Self {
        req_ctx.req_local.clone()
    }
}