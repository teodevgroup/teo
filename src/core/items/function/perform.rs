use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::callbacks::param::CallbackParam;
use crate::core::callbacks::types::callback::{CallbackArgument, CallbackResult};
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Clone)]
pub struct CallbackItem<T, O> {
    callback: Arc<dyn CallbackArgument<T, O>>
}

impl<T, O> Debug for CallbackItem<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("PerformItem");
        result.finish()
    }
}

impl<T, O> CallbackItem<T, O> {
    pub fn new<F>(f: F) -> CallbackItem<T, O> where
        T: Send + Sync + 'static,
        F: CallbackArgument<T, O> + 'static,
        O: Into<CallbackResult> + Send + Sync {
        return CallbackItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static, O: Into<CallbackResult> + Send + Sync + 'static> Item for CallbackItem<T, O> {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let cb = self.callback.clone();
        let param = CallbackParam {
            value: (&ctx).value.clone(),
            object: (&ctx).object.clone().unwrap().clone(),
            user_ctx: ctx.user_ctx(),
        };
        let result = cb.call(param).await.into();
        match result {
            CallbackResult::Result(result) => match result {
                Ok(_) => Ok(ctx),
                Err(error) => Err(ctx.unwrap_custom_error(error)),
            }
        }
    }
}

unsafe impl<T, O> Send for CallbackItem<T, O> {}
unsafe impl<T, O> Sync for CallbackItem<T, O> {}
