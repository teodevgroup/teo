use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::JsonPipeline;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::object::Object;

pub mod r#type;
pub mod builder;

#[derive(Clone)]
pub(crate) struct Action {
    transformers: Vec<JsonPipeline>,
}

impl Action {
    pub(crate) fn new() -> Self {
        Action { transformers: vec![] }
    }

    pub(crate) async fn transform(&self, value: &JsonValue, identity: Option<Object>) -> JsonValue {
        let mut value = value.clone();
        for transformer in self.transformers.iter() {
            value = transformer.call(&value, identity.clone()).await;
        }
        value
    }
}
