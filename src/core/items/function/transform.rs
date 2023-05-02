use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use std::sync::Arc;
use async_trait::async_trait;
use crate::core::callbacks::param::CallbackParam;

use crate::core::callbacks::types::transform::{TransformArgument, TransformResult};
use crate::core::result::Result;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;


#[derive(Clone)]
pub struct TransformItem<A0, O, R> {
    callback: Arc<dyn TransformArgument<A0, O, R>>
}

impl<A0, O, R> Debug for TransformItem<A0, O, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("TransformItem");
        result.finish()
    }
}

impl<A0, O, R> TransformItem<A0, O, R> {
    pub fn new<F>(f: F) -> TransformItem<A0, O, R> where
        A0: From<Value> + Send + Sync,
        O: Into<Value> + Send + Sync,
        R: Into<TransformResult<O>> + Send + Sync,
        F: TransformArgument<A0, O, R> + 'static {
        return TransformItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<A0: From<Value> + Send + Sync, O: Into<Value> + Send + Sync, R: Into<TransformResult<O>> + Send + Sync> Item for TransformItem<A0, O, R> {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let cb = self.callback.clone();
        let param = CallbackParam {
            value: (&ctx).value.clone(),
            object: (&ctx).object.unwrap().clone(),
            user_ctx: ctx.user_ctx(),
            _marker: PhantomData::default(),
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

unsafe impl<A0, O, R> Send for TransformItem<A0, O, R> {}
unsafe impl<A0, O, R> Sync for TransformItem<A0, O, R> {}
