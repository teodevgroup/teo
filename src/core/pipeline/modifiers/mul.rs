use async_trait::async_trait;
use crate::core::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct SubModifier {
    argument: Argument
}

impl SubModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for SubModifier {

    fn name(&self) -> &'static str {
        "sub"
    }

    async fn call(&self, context: Context) -> Context {
        let argument = self.argument.resolve(context).await;
        context.alter_value(context.value.clone() - argument)
    }
}
