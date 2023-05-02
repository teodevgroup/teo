use std::marker::PhantomData;
use crate::core::ctx::user::UserCtx;
use crate::prelude::{Object, Value};

pub struct CallbackParam<A0> {
    pub(crate) value: Value,
    pub(crate) object: Object,
    pub(crate) user_ctx: UserCtx,
    pub(crate) _marker: PhantomData<A0>,
}

pub trait ExtractValueFromCallbackParam<A0> {
    fn extract(param: &CallbackParam<A0>) -> Self;
}

pub trait ExtractFromCallbackParam<A0> {
    fn extract(param: &CallbackParam<A0>) -> Self;
}


impl<A0> ExtractValueFromCallbackParam<A0> for A0 where A0: From<Value> {
    fn extract(param: &CallbackParam<A0>) -> Self {
        param.value.clone().into()
    }
}

impl<A0> ExtractFromCallbackParam<A0> for Object {
    fn extract(param: &CallbackParam<A0>) -> Self {
        param.object.clone()
    }
}

impl<A0> ExtractFromCallbackParam<A0> for UserCtx {
    fn extract(param: &CallbackParam<A0>) -> Self {
        param.user_ctx.clone()
    }
}

pub struct IntoCallbackParam<A0> where A0: ExtractFromCallbackParam<A0> {
    param: A0
}