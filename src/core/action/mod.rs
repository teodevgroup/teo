use crate::core::json_pipeline::JsonPipeline;

use crate::core::object::Object;
use crate::prelude::Value;

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

    pub(crate) async fn transform(&self, value: &Value, identity: Option<Object>) -> Value {
        let mut value = value.clone();
        for transformer in self.transformers.iter() {
            value = transformer.call(&value, identity.clone()).await;
        }
        value
    }
}
