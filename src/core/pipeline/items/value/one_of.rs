use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

#[derive(Debug, Clone)]
pub struct OneOfModifier {
    argument: Value
}

impl OneOfModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for OneOfModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let arg = self.argument.resolve(ctx.clone()).await;
        let list = arg.as_vec().unwrap();
        if list.iter().find(|item| **item == arg).is_some() {
            ctx
        } else {
            ctx.invalid("Value is not in one of valid values.")
        }
    }
}
