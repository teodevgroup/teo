use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::callbacks::params::callback::{CallbackParam, ExtractFromCallbackParam, ExtractValueFromCallbackParam};
use crate::core::result::Result;

pub enum CallbackResult {
    Result(Result<()>)
}

impl From<()> for CallbackResult {
    fn from(_: ()) -> Self {
        CallbackResult::Result(Ok(()))
    }
}

impl From<Result<()>> for CallbackResult {
    fn from(result: Result<()>) -> Self {
        CallbackResult::Result(result)
    }
}

pub trait CallbackArgument<A, O: Into<CallbackResult>>: Send + Sync + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O>;
}

impl<A0, O, F, Fut> CallbackArgument<(A0,), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, Fut> CallbackArgument<(A0, A1), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1))
    }
}

impl<A0, A1, A2, O, F, Fut> CallbackArgument<(A0, A1, A2), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    A2: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1, A2) -> Fut + Sync + Send + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        let arg2: A2 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1, arg2))
    }
}

impl<A0, A1, A2, A3, O, F, Fut> CallbackArgument<(A0, A1, A2, A3), O> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    A2: ExtractFromCallbackParam + Send + Sync,
    A3: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1, A2, A3) -> Fut + Sync + Send + 'static,
    O: Into<CallbackResult> + Send + Sync,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, O> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        let arg2: A2 = ExtractFromCallbackParam::extract(&args);
        let arg3: A3 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1, arg2, arg3))
    }
}