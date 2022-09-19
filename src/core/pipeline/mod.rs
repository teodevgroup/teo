use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::object::Object;

pub mod builder;
pub mod context;
pub mod modifier;
pub mod modifiers;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub modifiers: Vec<Arc<dyn Modifier>>
}

impl Pipeline {

    pub(crate) fn has_any_modifier(&self) -> bool {
        self.modifiers.len() > 0
    }

    pub(crate) async fn process(&self, mut context: Context) -> Context {
        for modifier in &self.modifiers {
            context = modifier.call(context.clone()).await;
        }
        return context;
    }
}

// impl Clone for Pipeline {
//     fn clone(&self) -> Self {
//         Pipeline { modifiers: self.modifiers.clone() }
//     }
// }

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}
