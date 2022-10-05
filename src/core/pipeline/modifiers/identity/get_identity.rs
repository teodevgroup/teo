use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct GetIdentityModifier {}

impl GetIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for GetIdentityModifier {

    fn name(&self) -> &'static str {
        "getIdentity"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        match context.object.env().source().as_identity() {
            Some(o) => context.alter_value(Value::Object(o.clone())),
            None => context.alter_value(Value::Null),
        }
    }
}
