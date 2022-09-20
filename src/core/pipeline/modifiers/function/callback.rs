use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::object::Object;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::value::Value;

pub trait CallbackArgument<T: From<Value> + Send + Sync>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, ()>;
}

impl<T, F, Fut> CallbackArgument<T> for F where
T: From<Value> + Send + Sync,
F: Fn(T) -> Fut + Sync + Send,
Fut: Future<Output = ()> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, ()> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct CallbackModifier<T> {
    callback: Arc<dyn CallbackArgument<T>>
}

impl<T> Debug for CallbackModifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CallbackModifier");
        result.finish()
    }
}

impl<T> CallbackModifier<T> {
    pub fn new<F>(f: F) -> CallbackModifier<T> where
        T: From<Value> + Send + Sync,
        F: CallbackArgument<T> + 'static {
        return CallbackModifier {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync> Modifier for CallbackModifier<T> {

    fn name(&self) -> &'static str {
        "callback"
    }

    async fn call(&self, ctx: Context) -> Context {
        let cb = self.callback.clone();
        cb.call((&ctx).value.clone().into()).await;
        ctx
    }
}

unsafe impl<T> Send for CallbackModifier<T> {}
unsafe impl<T> Sync for CallbackModifier<T> {}
