use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::value::Value;

pub trait PerformArgument<T: From<Value> + Send + Sync>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, ()>;
}

impl<T, F, Fut> PerformArgument<T> for F where
T: From<Value> + Send + Sync,
F: Fn(T) -> Fut + Sync + Send,
Fut: Future<Output = ()> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, ()> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct PerformModifier<T> {
    callback: Arc<dyn PerformArgument<T>>
}

impl<T> Debug for PerformModifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("PerformModifier");
        result.finish()
    }
}

impl<T> PerformModifier<T> {
    pub fn new<F>(f: F) -> PerformModifier<T> where
        T: From<Value> + Send + Sync,
        F: PerformArgument<T> + 'static {
        return PerformModifier {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync> Modifier for PerformModifier<T> {

    fn name(&self) -> &'static str {
        "perform"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let cb = self.callback.clone();
        cb.call((&ctx).value.clone().into()).await;
        ctx
    }
}

unsafe impl<T> Send for PerformModifier<T> {}
unsafe impl<T> Sync for PerformModifier<T> {}
