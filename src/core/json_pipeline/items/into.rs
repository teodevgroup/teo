use async_trait::async_trait;
use crate::core::json_pipeline::context::JsonPipelineContext;
use crate::core::json_pipeline::json_path::json_get;
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
        let mut new_location = context.location().clone();
        new_location.push(self.key.clone());
        let new_value = if let Some(value) = context.value() {
            json_get(value, vec![self.key.clone()])
        } else {
            None
        }.cloned();
        let result = JsonPipelineContext::construct(new_value, new_location.clone(), context.object().clone(), context.stage(), context.identity().cloned());
        println!("see into result {:?}", result);
        result
    }
}
