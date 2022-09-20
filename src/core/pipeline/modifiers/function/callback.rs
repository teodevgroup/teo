use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::value::Value;

#[derive(Clone)]
pub struct CallbackModifier {
    callback: Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>
}

impl Debug for CallbackModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CallbackModifier");
        result.finish()
    }
}

impl CallbackModifier {
    pub fn new<F, I, Fut>(f: &'static F) -> CallbackModifier where
        F: Fn(I) -> Fut + Sync + Send + 'static,
        I: From<Value> + Send + Sync,
        Fut: Future<Output = ()> + Send + Sync {
        return CallbackModifier {
            callback: Arc::new(|value| Box::pin(async {
                f(I::from(value)).await
            }))
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
        cb((&ctx).value.clone()).await;
        ctx
    }
}

unsafe impl Send for CallbackModifier {}
unsafe impl Sync for CallbackModifier {}
