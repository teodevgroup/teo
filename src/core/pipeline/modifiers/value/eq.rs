use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct EqModifier {
    argument: Argument
}

impl EqModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for EqModifier {

    fn name(&self) -> &'static str {
        "eq"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let rhs = self.argument.resolve(ctx.clone()).await;
        if rhs == ctx.value {
            ctx
        } else {
            ctx.invalid("Value is not equal to rhs.")
        }
    }
}
