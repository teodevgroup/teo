use std::fmt::{Debug, Formatter};

use std::sync::Arc;
use async_trait::async_trait;
use crate::core::callbacks::params::callback::CallbackParam;
use crate::core::callbacks::types::transform::{TransformArgument, TransformResult};
use crate::core::result::Result;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use teo_teon::value::Value;

#[derive(Clone)]
pub struct TransformItem<A, O, R> {
    callback: Arc<dyn TransformArgument<A, O, R>>
}

impl<A, O, R> Debug for TransformItem<A, O, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("TransformItem");
        result.finish()
    }
}

impl<A, O, R> TransformItem<A, O, R> {
    pub fn new<F>(f: F) -> TransformItem<A, O, R> where
        A: Send + Sync,
        O: Into<Value> + Send + Sync,
        R: Into<TransformResult<O>> + Send + Sync,
        F: TransformArgument<A, O, R> + 'static {
        return TransformItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<A: Send + Sync + 'static, O: Into<Value> + Send + Sync + 'static, R: Into<TransformResult<O>> + Send + Sync + 'static> Item for TransformItem<A, O, R> {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let cb = self.callback.clone();
        let param = CallbackParam {
            value: (&ctx).value.clone(),
            object: (&ctx).object.clone().unwrap().clone(),
            user_ctx: ctx.user_ctx(),
            req: ctx.req(),
        };
        let value = cb.call(param).await;
        let function_result = value.into();
        match function_result {
            TransformResult::Value(value) => Ok(ctx.with_value(value.into())),
            TransformResult::Result(result) => match result {
                Ok(value) => Ok(ctx.with_value(value.into())),
                Err(error) => Err(ctx.unwrap_custom_error(error)),
            }
        }
    }
}

unsafe impl<A, O, R> Send for TransformItem<A, O, R> {}
unsafe impl<A, O, R> Sync for TransformItem<A, O, R> {}
