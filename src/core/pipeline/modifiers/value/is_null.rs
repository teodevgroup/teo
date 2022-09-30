use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsNullModifier {}

impl IsNullModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsNullModifier {

    fn name(&self) -> &'static str {
        "isNull"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if ctx.value.is_null() {
            ctx
        } else {
            ctx.invalid("Value is not null.")
        }
    }
}
