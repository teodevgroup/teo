use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::tson::Value;

#[derive(Debug, Clone)]
pub struct GtModifier {
    argument: Value
}

impl GtModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for GtModifier {

    fn name(&self) -> &'static str {
        "gt"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let rhs = self.argument.resolve(ctx.clone()).await;
        if ctx.value > rhs {
            ctx
        } else {
            ctx.invalid("Value is not greater than rhs.")
        }
    }
}
