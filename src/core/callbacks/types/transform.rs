use futures_util::future::BoxFuture;
use std::future::Future;
use crate::core::teon::Value;
use crate::core::error::Error;
use crate::core::result::Result;

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

pub trait TransformArgument<T: From<Value> + Send + Sync + Into<Value> + Send + Sync, R: Into<TransformResult<T>>>: Send + Sync {
    fn call(&self, args: T) -> BoxFuture<'static, R>;
}

impl<T, F, R, Fut> TransformArgument<T, R> for F where
    T: From<Value> + Send + Sync + Into<Value>,
    F: Fn(T) -> Fut + Sync + Send,
    R: Into<TransformResult<T>> + Send + Sync,
    Fut: Future<Output = R> + Send + 'static {
    fn call(&self, args: T) -> BoxFuture<'static, R> {
        Box::pin(self(args))
    }
}