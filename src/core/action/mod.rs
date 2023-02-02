use crate::core::object::Object;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::Pipeline;
use crate::prelude::Value;

pub mod r#type;
pub mod builder;

#[derive(Clone)]
pub(crate) struct Action {
    transformers: Vec<Pipeline>,
}

impl Action {

    pub(crate) fn new() -> Self {
        Action { transformers: vec![] }
    }

    pub(crate) async fn transform(&self, value: &Value, _identity: Option<Object>) -> Value {
        let value = value.clone();
        let mut context = Context::initial_state_with_value(value.clone());
        for transformer in self.transformers.iter() {
            context = transformer.process(context.clone()).await;
        }
        context.value
    }
}
