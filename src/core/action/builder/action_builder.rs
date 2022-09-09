use crate::core::action::Action;
use crate::core::json_pipeline::builder::JsonPipelineBuilder;

pub struct ActionBuilder {
    pub(crate) action: Action
}

impl ActionBuilder {
    pub(crate) fn new(action: Action) -> Self {
        ActionBuilder { action }
    }

    pub fn transform<F: Fn(&mut JsonPipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = JsonPipelineBuilder::new();
        build(&mut builder);
        self.action.transformers.push(builder.build());
        self
    }
}
