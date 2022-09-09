use async_trait::async_trait;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::json_pipeline::JsonPipelineItem;
use crate::core::pipeline::stage::Stage;

#[derive(Debug)]
pub(crate) struct IntoItem {
    key: String
}

impl IntoItem {
    pub(crate) fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl JsonPipelineItem for IntoItem {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext {

        todo!()
    }
}
