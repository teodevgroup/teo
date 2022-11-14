use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
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
impl Modifier for AddModifier {

    fn name(&self) -> &'static str {
        "add"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() + argument)
    }
}
