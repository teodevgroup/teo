use std::future::Future;
use futures_util::future::BoxFuture;
use crate::core::callbacks::compare_param::{CompareParam, ExtractFromCompareParam, ExtractOldValueFromCompareParam, ExtractNewValueFromCompareParam};
use crate::core::callbacks::types::validate::ValidateResult;
use crate::core::teon::Value;

pub trait CompareArgument<A, O: Into<ValidateResult>>: Send + Sync + 'static {
    fn call(&self, args: CompareParam) -> BoxFuture<'static, O>;
}

impl<V1, V2, O, F, Fut> CompareArgument<(V1, V2), O> for F where
    V1: ExtractOldValueFromCompareParam + Send + Sync,
    V2: ExtractNewValueFromCompareParam + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CompareParam) -> BoxFuture<'static, O> {
        let old: V1 = ExtractOldValueFromCompareParam::extract_old(&args);
        let new: V2 = ExtractNewValueFromCompareParam::extract_new(&args);
        Box::pin(self(old, new))
    }
}

impl<V1, V2, A1, O, F, Fut> CompareArgument<(V1, V2, A1), O> for F where
    V1: ExtractOldValueFromCompareParam + Send + Sync,
    V2: ExtractNewValueFromCompareParam + Send + Sync,
    A1: ExtractFromCompareParam + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2, A1) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CompareParam) -> BoxFuture<'static, O> {
        let old: V1 = ExtractOldValueFromCompareParam::extract_old(&args);
        let new: V2 = ExtractNewValueFromCompareParam::extract_new(&args);
        let a1 = ExtractFromCompareParam::extract(&args);
        Box::pin(self(old, new, a1))
    }
}

impl<V1, V2, A1, A2, O, F, Fut> CompareArgument<(V1, V2, A1, A2), O> for F where
    V1: ExtractOldValueFromCompareParam + Send + Sync,
    V2: ExtractNewValueFromCompareParam + Send + Sync,
    A1: ExtractFromCompareParam + Send + Sync,
    A2: ExtractFromCompareParam + Send + Sync,
    O: Into<ValidateResult> + Send + Sync,
    F: Fn(V1, V2, A1, A2) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = O> + Send + 'static {
    fn call(&self, args: CompareParam) -> BoxFuture<'static, O> {
        let old: V1 = ExtractOldValueFromCompareParam::extract_old(&args);
        let new: V2 = ExtractNewValueFromCompareParam::extract_new(&args);
        let a1 = ExtractFromCompareParam::extract(&args);
        let a2 = ExtractFromCompareParam::extract(&args);
        Box::pin(self(old, new, a1, a2))
    }
}