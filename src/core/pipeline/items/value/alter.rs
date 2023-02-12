use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct AlterItem {
    argument: Value
}

impl AlterItem {
    pub fn new(argument: Value) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for AlterItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let new_value = self.argument.resolve(ctx.clone()).await?;
        Ok(ctx.with_value(new_value))
    }
}
