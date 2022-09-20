use async_trait::async_trait;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::json_pipeline::JsonPipelineItem;


#[derive(Debug)]
pub(crate) struct TopItem { }

impl TopItem {
    pub(crate) fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl JsonPipelineItem for TopItem {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext {
        if context.location().is_empty() {
            context
        } else {
            let root = context.object();
            JsonPipelineContext::construct(Some(root.clone()), vec![], root.clone(), context.stage(), context.identity().cloned())
        }
    }
}
