use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct InvalidModifier { }

impl InvalidModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for InvalidModifier {

    fn name(&self) -> &'static str {
        "invalid"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        ctx.invalid("Value is invalid.")
    }
}
