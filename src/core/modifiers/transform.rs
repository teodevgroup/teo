// use std::fmt::{Debug, Formatter};
// use std::sync::Arc;
// use async_trait::async_trait;
// use crate::core::model_callback::PinFutureObj;
// use crate::core::modifier::Modifier;
// use crate::core::object::Object;
// use crate::core::pipeline::Pipeline;
// use crate::core::stage::Stage;
// use crate::core::stage::Stage::{Value as StageValue, ConditionTrue};
// use crate::core::value::Value;
// use crate::error::ActionError;
//
//
// #[derive(Clone)]
// pub struct TransformModifier {
//     callback: Arc<dyn Fn(Value, Object) -> PinFutureObj<Result<Value, String>> + Send + Sync>
// }
//
// impl Debug for TransformModifier {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut result = f.debug_struct("TransformModifier");
//         result.finish()
//     }
// }
//
// impl TransformModifier {
//     pub fn new(cb: Arc<dyn Fn(Value, Object) -> PinFutureObj<Result<Value, String>> + Send + Sync>) -> Self {
//         return TransformModifier {
//             callback: cb
//         };
//     }
// }
//
// #[async_trait]
// impl Modifier for TransformModifier {
//
//     fn name(&self) -> &'static str {
//         "transform"
//     }
//
//     async fn call(&self, stage: Stage, object: &Object) -> Stage {
//         let value = stage.value().unwrap();
//         let cb = self.callback.clone();
//         let result = cb(value, object.clone()).await;
//         match result {
//             Ok(value) => {
//                 Stage::Value(value)
//             },
//             Err(err_msg) => {
//                 Stage::Invalid(err_msg)
//             }
//         }
//     }
// }
//
// unsafe impl Send for TransformModifier {}
// unsafe impl Sync for TransformModifier {}