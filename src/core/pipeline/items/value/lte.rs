use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct LteModifier {
    argument: Value
}

impl LteModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for LteModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let rhs = self.argument.resolve(ctx.clone()).await?;
        if ctx.value <= rhs {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("lte: value is not less than or equal to rhs"))
        }
    }
}
