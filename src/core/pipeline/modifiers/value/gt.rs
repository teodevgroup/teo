use async_trait::async_trait;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct GtModifier {
    argument: FunctionArgument
}

impl GtModifier {
    pub fn new(argument: impl Into<FunctionArgument>) -> Self {
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
