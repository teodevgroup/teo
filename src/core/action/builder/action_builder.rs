use crate::core::action::Action;
use crate::prelude::PipelineBuilder;

pub struct ActionBuilder {
    pub(crate) action: Action
}

impl ActionBuilder {
    pub(crate) fn new(action: Action) -> Self {
        ActionBuilder { action }
    }

    pub fn transform<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        self.action.transformers.push(builder.build());
        self
    }
}
