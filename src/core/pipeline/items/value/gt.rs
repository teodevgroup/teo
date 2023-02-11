use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

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
impl Item for GtModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let rhs = self.argument.resolve(ctx.clone()).await;
        if ctx.value > rhs {
            ctx
        } else {
            ctx.invalid("Value is not greater than rhs.")
        }
    }
}
