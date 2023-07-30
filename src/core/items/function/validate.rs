use std::fmt::{Debug, Formatter};

use std::sync::Arc;
use async_trait::async_trait;
use crate::core::callbacks::params::callback::CallbackParam;

use crate::core::callbacks::types::validate::{ValidateArgument, ValidateResult};
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;
use crate::prelude::Error;

#[derive(Clone)]
pub struct ValidateItem<A, O> {
    callback: Arc<dyn ValidateArgument<A, O>>
}

impl<A, O> Debug for ValidateItem<A, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("ValidateItem");
        result.finish()
    }
}

impl<A, O> ValidateItem<A, O> {
    pub fn new<F>(f: F) -> ValidateItem<A, O> where
        A: Send + Sync + 'static,
        O: Into<ValidateResult> + Send + Sync,
        F: ValidateArgument<A, O> + 'static {
        return ValidateItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<A: Send + Sync + 'static, O: Into<ValidateResult> + Send + Sync + 'static> Item for ValidateItem<A, O> {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let cb = self.callback.clone();
        let param = CallbackParam {
            value: (&ctx).value.clone(),
            object: (&ctx).object.clone().unwrap().clone(),
            user_ctx: ctx.user_ctx(),
            req: ctx.req(),
        };
        let value = cb.call(param).await;
        let result = value.into();
        match result {
            ValidateResult::Validity(validity) => {
                if validity.is_valid() {
                    Ok(ctx)
                } else {
                    Err(ctx.unwrap_custom_error(Error::custom_validation_error(validity.invalid_reason().unwrap())))
                }
            }
            ValidateResult::Result(result) => {
                match result {
                    Ok(validity) => if validity.is_valid() {
                        Ok(ctx)
                    } else {
                        Err(ctx.unwrap_custom_error(Error::custom_validation_error(validity.invalid_reason().unwrap())))
                    },
                    Err(error) => Err(ctx.unwrap_custom_error(error)),
                }
            }
        }
    }
}

unsafe impl<A, O> Send for ValidateItem<A, O> {}
unsafe impl<A, O> Sync for ValidateItem<A, O> {}
