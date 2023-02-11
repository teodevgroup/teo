use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct AddModifier {
    argument: Value
}

impl AddModifier {
    pub fn new(argument: Value) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for AddModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        Ok(ctx.with_value_result(ctx.get_value() + argument)?)
    }
}
