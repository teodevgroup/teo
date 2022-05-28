use std::fmt::{Debug};
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;
use crate::core::object::Object;


#[async_trait]
pub trait FnArgument: Debug + Send + Sync {
    fn name(&self) -> String;
    async fn call(&self, value: Value, object: Object) -> Stage;
}

#[derive(Debug, Clone)]
pub enum Argument {
    ValueArgument(Value),
    PipelineArgument(Pipeline),
    FunctionArgument(Arc<dyn FnArgument>),
}


//
// impl Debug for Argument {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }
