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

pub trait CallbackArgument: Send + Sync {
    fn call(&self, args: Object) -> BoxFuture<'static, ()>;
}

impl<F, Fut> CallbackArgument for F where
F: Fn(Object) -> Fut + Sync + Send,
Fut: Future<Output = ()> + Send + Sync + 'static {
    fn call(&self, args: Object) -> BoxFuture<'static, ()> {
        Box::pin(self(args))
    }
}

#[derive(Clone)]
pub struct CallbackModifier {
    callback: Arc<dyn CallbackArgument>
}

impl Debug for CallbackModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CallbackModifier");
        result.finish()
    }
}

impl CallbackModifier {
    pub fn new<F>(f: F) -> CallbackModifier where
        F: CallbackArgument + 'static {
        return CallbackModifier {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl Modifier for CallbackModifier {

    fn name(&self) -> &'static str {
        "callback"
    }

    async fn call(&self, ctx: Context) -> Context {
        let cb = self.callback.clone();
        cb.call((&ctx).value.clone().as_object().unwrap().clone()).await;
        ctx
    }
}

unsafe impl Send for CallbackModifier {}
unsafe impl Sync for CallbackModifier {}
