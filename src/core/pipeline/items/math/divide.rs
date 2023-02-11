use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;

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
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.with_value(context.value.clone() * argument)
    }
}
