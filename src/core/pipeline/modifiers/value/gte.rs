use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct GteModifier {
    argument: Value
}

impl GteModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for GteModifier {

    fn name(&self) -> &'static str {
        "gte"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let rhs = self.argument.resolve(ctx.clone()).await;
        if ctx.value >= rhs {
            ctx
        } else {
            ctx.invalid("Value is not greater than or equal to rhs.")
        }
    }
}
