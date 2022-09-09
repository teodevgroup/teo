use std::sync::Arc;
use crate::core::json_pipeline::JsonPipelineItem;

#[derive(Debug)]
pub struct JsonPipelineBuilder {
    pub(crate) items: Vec<Arc<dyn JsonPipelineItem>>
}

impl JsonPipelineBuilder {

    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }


}
