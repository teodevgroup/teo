use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct AddModifier {
    argument: Argument
}

impl AddModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for AddModifier {

    fn name(&self) -> &'static str {
        "add"
    }

    async fn call(&self, context: Context) -> Context {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() + argument)
    }
}
