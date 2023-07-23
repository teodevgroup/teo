use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::app::routes::req::Req;
use crate::app::routes::res::Res;
use crate::core::result::Result;

pub struct ActionCtx {
    pub(crate) req: Req,
}

pub trait ActionHandler: Send + Sync {
    fn call(&self, ctx: &mut ActionCtx) -> BoxFuture<'static, Result<Res>>;
}

impl<F, Fut> ActionHandler for F where
    F: Fn(&mut ActionCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<Res>> + Send + 'static {
    fn call(&self, ctx: &mut ActionCtx) -> BoxFuture<'static, Result<Res>> {
        Box::pin(self(ctx))
    }
}

pub(crate) struct ActionHandlerDef {
    pub(crate) group: &'static str,
    pub(crate) name: &'static str,
    pub(crate) f: Arc<dyn ActionHandler>,
}