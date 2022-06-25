use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use futures_util::future::BoxFuture;
use crate::core::object::Object;
use crate::core::value::Value;

pub(crate) type PinFutureObj<Output> = Pin<Box<dyn Future<Output = Output>>>;

pub(crate) type PinFutureObjSendSync<Output> = Pin<Box<dyn Future<Output = Output> + Send + Sync>>;
