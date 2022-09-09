use async_trait::async_trait;
use crate::core::json_pipeline::JsonPipelineItem;
use crate::core::pipeline::stage::Stage;

#[derive(Debug)]
pub(crate) struct GetItem {
    key: String
}

impl GetItem {
    pub(crate) fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl JsonPipelineItem for GetItem {
    async fn call(&self, stage: Stage) -> Stage {
        todo!()
    }
}
