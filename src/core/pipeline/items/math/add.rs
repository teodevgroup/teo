use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct AddModifier {
    argument: Value
}

impl AddModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for AddModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.with_value(context.value.clone() + argument)
    }
}
