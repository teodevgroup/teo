use std::marker::PhantomData;
use crate::core::ctx::user::UserCtx;
use crate::prelude::{Object, Value};

pub struct CallbackParam<A> {
    pub value: Value,
    pub object: Object,
    pub user_ctx: UserCtx,
    pub(crate) _marker: PhantomData<A>,
}

pub trait ExtractValueFromCallbackParam<A> {
    fn extract(param: &CallbackParam<A>) -> Self;
}

pub trait ExtractFromCallbackParam<A> {
    fn extract(param: &CallbackParam<A>) -> Self;
}

impl<A> ExtractValueFromCallbackParam<A> for A where A: From<Value> {
    fn extract(param: &CallbackParam<A>) -> Self {
        param.value.clone().into()
    }
}

impl<A> ExtractFromCallbackParam<A> for Object {
    fn extract(param: &CallbackParam<A>) -> Self {
        param.object.clone()
    }
}

impl<A> ExtractFromCallbackParam<A> for UserCtx {
    fn extract(param: &CallbackParam<A>) -> Self {
        param.user_ctx.clone()
    }
}
