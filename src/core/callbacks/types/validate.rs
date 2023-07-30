use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::callbacks::params::callback::{CallbackParam, ExtractFromCallbackParam, ExtractValueFromCallbackParam};
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

pub trait ValidateArgument<A, O: Into<ValidateResult>>: Send + Sync + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O>;
}

impl<A0, O, F, Fut> ValidateArgument<(A0,), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<ValidateResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, Fut> ValidateArgument<(A0, A1), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + 'static,
    O: Into<ValidateResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, Fut> ValidateArgument<(A0, A1, A2), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    A2: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + 'static,
    O: Into<ValidateResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        let arg2: A2 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1, arg2))
    }
}