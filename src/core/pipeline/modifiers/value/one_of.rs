use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct OneOfModifier {
    argument: Argument
}

impl OneOfModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for OneOfModifier {

    fn name(&self) -> &'static str {
        "oneOf"
    }

    async fn call(&self, ctx: Context) -> Context {
        let arg = self.argument.resolve(ctx.clone()).await;
        let list = arg.as_vec().unwrap();
        if list.iter().find(|item| **item == arg).is_some() {
            ctx
        } else {
            ctx.invalid("Value is not in one of valid values.")
        }
    }
}
