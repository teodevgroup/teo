use std::future::Future;
use std::pin::Pin;





pub(crate) type PinFutureObj<Output> = Pin<Box<dyn Future<Output = Output>>>;

pub(crate) type PinFutureObjSendSync<Output> = Pin<Box<dyn Future<Output = Output> + Send + Sync>>;
