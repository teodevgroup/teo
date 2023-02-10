use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct ExistsModifier {}

impl ExistsModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for ExistsModifier {

    fn name(&self) -> &'static str {
        "exists"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if ctx.value.is_null() {
            ctx.invalid("Value does not exist.")
        } else {
            ctx
        }
    }
}
