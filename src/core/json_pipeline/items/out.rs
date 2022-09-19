use async_trait::async_trait;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::json_pipeline::json_path::json_get;
use crate::core::json_pipeline::JsonPipelineItem;
use crate::core::pipeline::context::Context;

#[derive(Debug)]
pub(crate) struct OutItem { }

impl OutItem {
    pub(crate) fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl JsonPipelineItem for OutItem {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext {
        if context.location().is_empty() {
            context
        } else {
            let root = context.object();
            let mut new_location = context.location().clone();
            new_location.pop();
            let new_value = json_get(root, new_location.clone()).cloned();
            JsonPipelineContext::construct(new_value, new_location.clone(), root.clone(), context.stage(), context.identity().cloned())
        }
    }
}
