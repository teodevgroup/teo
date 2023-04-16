use std::fmt::{Debug, Formatter};

use std::sync::Arc;
use async_trait::async_trait;

use crate::core::callbacks::types::validate::{ValidateArgument, ValidateResult};
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::prelude::Error;

#[derive(Clone)]
pub struct ValidateItem<T, O> {
    callback: Arc<dyn ValidateArgument<T, O>>
}

impl<T, O> Debug for ValidateItem<T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("ValidateItem");
        result.finish()
    }
}

impl<T, O> ValidateItem<T, O> {
    pub fn new<F>(f: F) -> ValidateItem<T, O> where
        T: From<Value> + Send + Sync,
        O: Into<ValidateResult> + Send + Sync,
        F: ValidateArgument<T, O> + 'static {
        return ValidateItem {
            callback: Arc::new(f)
        }
    }
}

#[async_trait]
impl<T: From<Value> + Send + Sync, O: Into<ValidateResult> + Send + Sync> Item for ValidateItem<T, O> {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let cb = self.callback.clone();
        let value = cb.call((&ctx).value.clone().into()).await;
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

unsafe impl<T, O> Send for ValidateItem<T, O> {}
unsafe impl<T, O> Sync for ValidateItem<T, O> {}
