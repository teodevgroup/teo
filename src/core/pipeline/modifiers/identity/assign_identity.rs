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

    async fn call(&self, context: Context) -> Context {
        context.object.set_identity(context.identity.clone());
        context
    }
}
