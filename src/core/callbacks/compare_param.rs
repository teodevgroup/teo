use std::marker::PhantomData;
use crate::core::ctx::user::UserCtx;
use crate::prelude::{Object, Value};

pub struct CompareParam {
    pub value_old: Value,
    pub value_new: Value,
    pub object: Object,
    pub user_ctx: UserCtx,
}

pub trait ExtractOldValueFromCompareParam {
    fn extract_old(param: &CompareParam) -> Self;
}

pub trait ExtractNewValueFromCompareParam {
    fn extract_new(param: &CompareParam) -> Self;
}

pub trait ExtractFromCompareParam {
    fn extract(param: &CompareParam) -> Self;
}

impl<A> ExtractOldValueFromCompareParam for A where A: From<Value> {
    fn extract_old(param: &CompareParam) -> Self {
        param.value_old.clone().into()
    }
}

impl<A> ExtractNewValueFromCompareParam for A where A: From<Value> {
    fn extract_new(param: &CompareParam) -> Self {
        param.value_new.clone().into()
    }
}

impl ExtractFromCompareParam for Object {
    fn extract(param: &CompareParam) -> Self {
        param.object.clone()
    }
}

impl ExtractFromCompareParam for UserCtx {
    fn extract(param: &CompareParam) -> Self {
        param.user_ctx.clone()
    }
}
