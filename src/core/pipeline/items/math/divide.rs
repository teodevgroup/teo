use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct DivideModifier {
    argument: Value
}

impl DivideModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for DivideModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(ctx.with_value_result(ctx.get_value() / argument)?)
    }
}
