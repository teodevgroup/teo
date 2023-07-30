use crate::core::ctx::user::UserCtx;
use crate::prelude::{Object, Req, Value};

pub struct CallbackParam {
    pub value: Value,
    pub object: Object,
    pub user_ctx: UserCtx,
    pub req: Option<Req>,
}

pub trait ExtractValueFromCallbackParam {
    fn extract(param: &CallbackParam) -> Self;
}

pub trait ExtractFromCallbackParam {
    fn extract(param: &CallbackParam) -> Self;
}

impl<A> ExtractValueFromCallbackParam for A where A: From<Value> {
    fn extract(param: &CallbackParam) -> Self {
        param.value.clone().into()
    }
}

impl ExtractFromCallbackParam for Object {
    fn extract(param: &CallbackParam) -> Self {
        param.object.clone()
    }
}

impl ExtractFromCallbackParam for UserCtx {
    fn extract(param: &CallbackParam) -> Self {
        param.user_ctx.clone()
    }
}

impl ExtractFromCallbackParam for Option<Req> {
    fn extract(param: &CallbackParam) -> Self {
        param.req.clone()
    }
}
