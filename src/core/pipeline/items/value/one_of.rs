use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct OneOfModifier {
    argument: Value
}

impl OneOfModifier {
    pub fn new(argument: Value) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for OneOfModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let arg = self.argument.resolve(ctx.clone()).await?;
        let list = arg.as_vec().unwrap();
        if list.iter().find(|item| **item == arg).is_some() {
            Ok(ctx)
        } else {
            Err(ctx.with_invalid("oneOf: value is not one of valid ones"))
        }
    }
}
