use std::sync::Arc;
use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::pipeline::stage::Stage;


pub(crate) mod builder;
mod items;

#[async_trait]
pub(crate) trait JsonPipelineItem: Debug + Send + Sync {
    async fn call(&self, stage: Stage) -> Stage;
}

#[derive(Debug)]
pub(crate) struct JsonPipeline {
    pub(crate) items: Vec<Arc<dyn JsonPipelineItem>>
}
