use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::model_callback::{PinFutureObjSendSync};
use crate::core::modifier::Modifier;
use crate::core::object::Object;

use crate::core::stage::Stage;

use crate::core::value::Value;
use crate::core::error::ActionError;

#[derive(Clone)]
pub struct CompareModifier {
    callback: Arc<dyn Fn(Value, Value) -> PinFutureObjSendSync<Result<(), ActionError>> + Send + Sync>,
}

impl Debug for CompareModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("CompareModifier");
        result.finish()
    }
}

impl CompareModifier {
    pub fn new<F, I, O, Fut>(f: &'static F) -> CompareModifier where
        F: Fn(I, I) -> Fut + Sync + Send + 'static,
        I: From<Value> + Send + Sync,
        Fut: Future<Output = Result<(), ActionError>> + Send + Sync {
        return CompareModifier {
            callback: Arc::new(|old, new| Box::pin(async {
                f(I::from(old), I::from(new)).await
            }))
        }
    }
}

#[async_trait]
impl Modifier for CompareModifier {

    fn name(&self) -> &'static str {
        "compare"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        let new = stage.value().unwrap();
        let cb = self.callback.clone();
        let _result = cb(new.clone(), new.clone()).await;
        Stage::Value(new)
    }
}

unsafe impl Send for CompareModifier {}
unsafe impl Sync for CompareModifier {}
