use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ModularModifier {
    argument: Value
}

impl ModularModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for ModularModifier {

    fn name(&self) -> &'static str {
        "modular"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        context.alter_value(context.value.clone() % argument)
    }
}
