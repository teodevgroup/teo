use std::sync::Arc;
use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


pub(crate) mod builder;
mod items;
pub(crate) mod context;
mod json_path;

#[async_trait]
pub(crate) trait JsonPipelineItem: Debug + Send + Sync {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext;
}

#[derive(Debug, Clone)]
pub(crate) struct JsonPipeline {
    pub(crate) items: Vec<Arc<dyn JsonPipelineItem>>
}

impl JsonPipeline {
    pub(crate) async fn call(&self, value: &JsonValue, identity: Option<Object>) -> JsonValue {
        let mut context = JsonPipelineContext::new(value.clone(), identity.clone());
        for item in self.items.iter() {
            context = item.call(context).await;
        }
        context.object().clone()
    }
}
