pub mod builder;
pub mod context;
pub mod modifier;
pub mod modifiers;

use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub modifiers: Vec<Arc<dyn Modifier>>
}

impl Pipeline {

    pub(crate) fn new() -> Self {
        Self { modifiers: vec![] }
    }

    pub(crate) fn has_any_modifier(&self) -> bool {
        self.modifiers.len() > 0
    }

    pub(crate) async fn process<'a>(&self, mut context: Context<'a>) -> Context<'a> {
        for modifier in &self.modifiers {
            context = modifier.call(context.clone()).await;
            if !context.is_valid() {
                break
            }
        }
        return context;
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl PartialEq for Pipeline {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
