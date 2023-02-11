use async_trait::async_trait;

use crate::core::pipeline::item::Item;


use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct AssignIdentityModifier {}

impl AssignIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for AssignIdentityModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        context
    }
}
