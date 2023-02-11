use async_trait::async_trait;

use crate::core::pipeline::item::Item;


use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct RequireIdentityModifier {}

impl RequireIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for RequireIdentityModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        if context.object.as_ref().unwrap().action_source().as_identity().is_some() {
            context
        } else {
            context.invalid("Identity is not present.")
        }
    }
}
