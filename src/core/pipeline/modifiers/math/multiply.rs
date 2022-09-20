use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct MultiplyModifier {
    argument: Argument
}

impl MultiplyModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for MultiplyModifier {

    fn name(&self) -> &'static str {
        "multiply"
    }

    async fn call(&self, context: Context) -> Context {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() * argument)
    }
}
