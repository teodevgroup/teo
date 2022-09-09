use crate::core::action::Action;
use crate::core::json_pipeline::builder::JsonPipelineBuilder;

pub struct ActionBuilder {
    pub(crate) action: Action
}

impl ActionBuilder {
    pub(crate) fn new(action: Action) -> Self {
        ActionBuilder { action }
    }

    pub fn transform<F: Fn(&mut JsonPipelineBuilder)>(&mut self, _build: F) -> &mut Self {
        self
    }
}
