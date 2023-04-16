use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct GtItem {
    argument: Value
}

impl GtItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for GtItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let rhs = self.argument.resolve(ctx.clone()).await?;
        if ctx.value > rhs {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("gt: value is not greater than rhs"))
        }
    }
}
