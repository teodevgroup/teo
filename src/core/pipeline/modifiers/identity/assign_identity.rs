use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct AssignIdentityModifier {}

impl AssignIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for AssignIdentityModifier {

    fn name(&self) -> &'static str {
        "assignIdentity"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        context.alter_identity(Some(context.object.clone()));
        context
    }
}
