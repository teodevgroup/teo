pub mod ctx;
pub mod item;
pub mod items;

use std::sync::Arc;
use crate::core::result::Result;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

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

    pub(crate) async fn process(&self, mut ctx: Ctx<'_>) -> Result<Value> {
        for item in &self.items {
            ctx = item.call(ctx.clone()).await?;
        }
        Ok(ctx.value)
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl PartialEq for Pipeline {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
