use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::model_callback::{PinFutureObjSendSync};
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;

use crate::core::pipeline::stage::Stage;

use crate::core::value::Value;


#[derive(Clone)]
pub struct TransformModifier {
    callback: Arc<dyn Fn(Value) -> PinFutureObjSendSync<Value> + Send + Sync>,
}

impl Debug for TransformModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("TransformModifier");
        result.finish()
    }
}

impl TransformModifier {
    pub fn new<F, I, O, Fut>(f: &'static F) -> TransformModifier where F: Fn(I) -> Fut + Sync + Send + 'static, I: From<Value> + Send + Sync, O: Into<Value>, Fut: Future<Output = O> + Send + Sync {
        return TransformModifier {
            callback: Arc::new(|value| Box::pin(async {
                f(I::from(value)).await.into()
            }))
        }
    }
}

#[async_trait]
impl Modifier for TransformModifier {

    fn name(&self) -> &'static str {
        "transform"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        let value = stage.value().unwrap();
        let cb = self.callback.clone();
        let result = cb(value.clone()).await;
        Stage::Value(result)
    }
}

unsafe impl Send for TransformModifier {}
unsafe impl Sync for TransformModifier {}
