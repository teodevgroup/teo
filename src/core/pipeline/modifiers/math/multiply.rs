use async_trait::async_trait;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct MultiplyModifier {
    argument: FunctionArgument
}

impl MultiplyModifier {
    pub fn new(argument: impl Into<FunctionArgument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for MultiplyModifier {

    fn name(&self) -> &'static str {
        "multiply"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() * argument)
    }
}
