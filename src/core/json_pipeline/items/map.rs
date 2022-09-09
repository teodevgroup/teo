use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::{JsonPipeline, JsonPipelineItem};
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::pipeline::stage::Stage;

#[derive(Debug)]
pub(crate) struct MapItem {
    pipeline: JsonPipeline,
}

impl MapItem {
    pub(crate) fn new(pipeline: JsonPipeline) -> Self {
        Self { pipeline }
    }
}

#[async_trait]
impl JsonPipelineItem for MapItem {
    async fn call(&self, context: JsonPipelineContext) -> JsonPipelineContext {
        todo!()
    }
}
