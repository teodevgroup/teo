use std::future::Future;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::core::result::Result;

pub struct ActionCtx {

}

pub trait ActionHandler: Send + Sync {
    fn call(&self, ctx: &mut ActionCtx) -> BoxFuture<'static, Result<()>>;
}

impl<F, Fut> ActionHandler for F where
    F: Fn(&mut ActionCtx) -> Fut + Sync + Send,
    Fut: Future<Output = Result<()>> + Send + 'static {
    fn call(&self, ctx: &mut ActionCtx) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub(crate) struct ActionHandlerDef {
    pub(crate) group: &'static str,
    pub(crate) name: &'static str,
    pub(crate) f: Arc<dyn ActionHandler>,
}