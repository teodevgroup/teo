use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct SubtractModifier {
    argument: Value
}

impl SubtractModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for SubtractModifier {

    fn name(&self) -> &'static str {
        "subtract"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() - argument)
    }
}
