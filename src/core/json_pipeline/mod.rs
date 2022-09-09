use std::sync::Arc;
use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::pipeline::stage::Stage;


pub(crate) mod builder;
mod items;
mod context;
mod json_path;

#[async_trait]
pub(crate) trait JsonPipelineItem: Debug + Send + Sync {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext;
}

#[derive(Debug)]
pub(crate) struct JsonPipeline {
    pub(crate) items: Vec<Arc<dyn JsonPipelineItem>>
}
