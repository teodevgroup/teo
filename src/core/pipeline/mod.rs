pub mod ctx;
pub mod item;
pub mod items;

use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub items: Vec<Arc<dyn Item>>
}

impl Pipeline {

    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }

    pub(crate) fn has_any_items(&self) -> bool {
        self.items.len() > 0
    }

    pub(crate) async fn process<'a>(&self, mut ctx: Ctx<'a>) -> Ctx<'a> {
        for item in &self.items {
            ctx = item.call(ctx.clone()).await;
            if !ctx.state.is_value() {
                break
            }
        }
        return ctx;
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl PartialEq for Pipeline {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
