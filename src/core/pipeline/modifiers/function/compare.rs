use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::{Context};
use crate::core::pipeline::context::validity::Validity;
use crate::core::value::Value;

pub trait CompareArgument<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync>: Send + Sync {
    fn call(&self, old: T, new: T) -> BoxFuture<'static, O>;
}

impl<T, O, F, Fut> CompareArgument<T, O> for F where
    T: From<Value> + Send + Sync,
    O: Into<Validity> + Send + Sync,
    F: Fn(T, T) -> Fut + Sync + Send,
    Fut: Future<Output = O> + Send + Sync + 'static {
    fn call(&self, old: T, new: T) -> BoxFuture<'static, O> {
        Box::pin(self(old, new))
    }
}

#[derive(Clone)]
pub struct CompareModifier<T, O> {
    callback: Arc<dyn CompareArgument<T, O>>
}

impl<T, O> Debug for CompareModifier<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CompareModifier");
        result.finish()
    }
}

impl<T, O> CompareModifier<T, O> {
    pub fn new<F>(f: F) -> CompareModifier<T, O> where
        T: From<Value> + Send + Sync,
        O: Into<Validity> + Send + Sync,
        F: CompareArgument<T, O> + 'static {
        return CompareModifier {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync> Modifier for CompareModifier<T, O> {

    fn name(&self) -> &'static str {
        "compare"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if ctx.object.is_new() {
            return ctx;
        }
        if ctx.key_path.len() != 1 {
            return ctx.invalid("Compare can only be used on first level fields.");
        }
        let key = ctx.key_path[0].as_key().unwrap();
        let previous_value = ctx.object.get_previous_value(key);
        if let Ok(previous_value) = previous_value {
            let current_value = (&ctx).value.clone();
            if previous_value == current_value {
                return ctx.clone();
            }
            let cb = self.callback.clone();
            let value = cb.call(previous_value.into(), current_value.into()).await;
            let validity = value.into();
            if validity.is_valid() {
                ctx.clone()
            } else {
                ctx.invalid(validity.reason().unwrap())
            }
        } else {
            ctx.clone()
        }
    }
}

unsafe impl<T, O> Send for CompareModifier<T, O> {}
unsafe impl<T, O> Sync for CompareModifier<T, O> {}
