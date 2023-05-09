use futures_util::future::BoxFuture;
use std::future::Future;
use crate::core::callbacks::param::{CallbackParam, ExtractFromCallbackParam, ExtractValueFromCallbackParam};
use crate::core::teon::Value;
use crate::core::error::Error;
use crate::core::result::Result;
use crate::prelude::{Object, UserCtx};

pub enum TransformResult<T> where T: Into<Value> {
    Value(T),
    Result(Result<T>),
}

impl<T> From<T> for TransformResult<T> where T: Into<Value> {
    fn from(value: T) -> Self {
        TransformResult::Value(value)
    }
}

impl<T, U> From<std::result::Result<T, U>> for TransformResult<T> where T: Into<Value>, U: Into<Error> {
    fn from(result: std::result::Result<T, U>) -> Self {
        match result {
            Ok(t) => TransformResult::Result(Ok(t)),
            Err(err) => TransformResult::Result(Err(err.into())),
        }
    }
}

pub trait TransformArgument<A, O: Into<Value>, R: Into<TransformResult<O>>>: Send + Sync + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, R>;
}

impl<A0, O, F, R, Fut> TransformArgument<(A0,), O, R> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    F: Fn(A0) -> Fut + Sync + Send + Clone + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, R> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        Box::pin(self(value))
    }
}

impl<A0, A1, O, F, R, Fut> TransformArgument<(A0, A1), O, R> for F where
    A0: ExtractValueFromCallbackParam + Send + Sync,
    A1: ExtractFromCallbackParam + Send + Sync,
    F: Fn(A0, A1) -> Fut + Sync + Send + 'static,
    O: Into<Value> + Sync + Send,
    R: Into<TransformResult<O>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, args: CallbackParam) -> BoxFuture<'static, R> {
        let value: A0 = ExtractValueFromCallbackParam::extract(&args);
        let arg1: A1 = ExtractFromCallbackParam::extract(&args);
        Box::pin(self(value, arg1))
    }
}
//
// impl<A0, A1, A2, O, F, R, Fut> TransformArgument<A0, O, R> for F where
//     A0: ExtractValueFromCallbackParam<A0> + Send + Sync,
//     A1: ExtractFromCallbackParam<A0> + Send + Sync,
//     A2: ExtractFromCallbackParam<A0> + Send + Sync,
//     F: Fn(A0, A1, A2) -> Fut + Sync + Send,
//     O: Into<Value> + Sync + Send,
//     R: Into<TransformResult<O>> + Send + Sync,
//     Fut: Future<Output = R> + Send + 'static {
//     fn call(&self, args: CallbackParam<A0>) -> BoxFuture<'static, R> {
//         let value: A0 = ExtractValueFromCallbackParam::extract(&args);
//         let arg1: A1 = ExtractFromCallbackParam::extract(&args);
//         let arg2: A2 = ExtractFromCallbackParam::extract(&args);
//         Box::pin(self(value, arg1, arg2))
//     }
// }