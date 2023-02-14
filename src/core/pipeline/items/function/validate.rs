use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::prelude::Error;
use self::Validity::*;

#[derive(Clone)]
pub enum Validity {
    Valid,
    Invalid(String)
}

impl Validity {
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Valid => true,
            _ => false,
        }
    }

    pub(crate) fn invalid_reason(&self) -> Option<&str> {
        match self {
            Invalid(reason) => Some(&reason),
            _ => None,
        }
    }
}

impl From<&str> for Validity {
    fn from(reason: &str) -> Self {
        Invalid(reason.to_string())
    }
}

impl From<String> for Validity {
    fn from(reason: String) -> Self {
        Invalid(reason)
    }
}

impl From<bool> for Validity {
    fn from(valid: bool) -> Self {
        match valid {
            true => Valid,
            false => Invalid("value is invalid".to_owned())
        }
    }
}

impl From<()> for Validity {
    fn from(_: ()) -> Self {
        Valid
    }
}

impl From<Option<String>> for Validity {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Validity::Invalid(v),
            None => Validity::Valid,
        }
    }
}

impl From<Option<&str>> for Validity {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(v) => Validity::Invalid(v.to_owned()),
            None => Validity::Valid,
        }
    }
}

pub enum ValidateResult {
    Validity(Validity),
    Result(Result<Validity>),
}

impl<T> From<T> for ValidateResult where T: Into<Validity> {
    fn from(value: T) -> Self {
        ValidateResult::Validity(value.into())
    }
}

impl<T, U> From<std::result::Result<T, U>> for ValidateResult where T: Into<Validity>, U: Into<Error> {
    fn from(value: std::result::Result<T, U>) -> Self {
        match value {
            Ok(t) => ValidateResult::Result(Ok(t.into())),
            Err(e) => ValidateResult::Result(Err(e.into())),
        }
    }
}

pub trait ValidateArgument<T: From<Value> + Send + Sync, O: Into<ValidateResult> + Send + Sync>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, O>;
}

impl<T, O, F, Fut> ValidateArgument<T, O> for F where
    T: From<Value> + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(T) -> Fut + Sync + Send,
    Fut: Future<Output = O> + Send + Sync + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, O> {
        Box::pin(self(args))
    }
}

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
