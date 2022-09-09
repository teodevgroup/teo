use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::json_pipeline::json_path::{json_has, json_set};
use crate::core::json_pipeline::JsonPipelineItem;
use crate::core::pipeline::stage::Stage;

#[derive(Debug)]
pub(crate) struct SetDefaultItem {
    key: String,
    value: JsonValue,
}

impl SetDefaultItem {
    pub(crate) fn new(key: impl Into<String>, value: JsonValue) -> Self {
        Self { key: key.into(), value }
    }
}

#[async_trait]
impl JsonPipelineItem for SetDefaultItem {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext {
        let mut new_value = context.value().cloned();
        let new_value = if let Some(new_value) = new_value {
            let path = vec![self.key.clone()];
            if json_has(&new_value, path.clone()) {
                Some(json_set(&new_value, path.clone(), self.value.clone()))
            } else {
                Some(new_value)
            }
        } else {
            None
        };
        JsonPipelineContext::construct(new_value, context.location().clone(), context.object().clone(), context.stage(), context.identity().cloned())
    }
}
