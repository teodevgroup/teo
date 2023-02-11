use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::{Ctx};
use crate::core::pipeline::ctx::validity::Validity;
use crate::core::teon::Value;
use crate::core::result::Result;

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
impl<T: From<Value> + Send + Sync, O: Into<Validity> + Send + Sync> Item for CompareModifier<T, O> {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        if ctx.get_object()?.is_new() {
            return Ok(ctx);
        }
        if ctx.path.len() != 1 {
            return Err(ctx.internal_server_error("compare: used on nested level fields."));
        }
        let key = ctx.path[0].as_key().unwrap();
        let previous_value = ctx.object.as_ref().unwrap().get_previous_value(key);
        if let Ok(previous_value) = previous_value {
            let current_value = (&ctx).value.clone();
            if previous_value == current_value {
                return Ok(ctx.clone());
            }
            let cb = self.callback.clone();
            let value = cb.call(previous_value.into(), current_value.into()).await;
            let validity = value.into();
            if validity.is_valid() {
                Ok(ctx.clone())
            } else {
                Err(ctx.internal_server_error(validity.invalid_reason().unwrap()))
            }
        } else {
            Ok(ctx.clone())
        }
    }
}

unsafe impl<T, O> Send for CompareModifier<T, O> {}
unsafe impl<T, O> Sync for CompareModifier<T, O> {}
