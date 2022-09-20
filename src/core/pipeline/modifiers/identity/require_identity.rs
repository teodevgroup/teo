use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct RequireIdentityModifier {}

impl RequireIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for RequireIdentityModifier {

    fn name(&self) -> &'static str {
        "requireIdentity"
    }

    async fn call(&self, context: Context) -> Context {
        if context.identity.is_some() {
            context
        } else {
            context.invalid("Identity is not present.")
        }
    }
}
