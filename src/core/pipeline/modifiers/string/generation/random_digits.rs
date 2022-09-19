use async_trait::async_trait;
use random_string::generate;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct RandomDigitsModifier {
    len: Argument
}

impl RandomDigitsModifier {
    pub fn new(len: impl Into<Argument>) -> Self {
        return RandomDigitsModifier {
            len: len.into()
        };
    }
}

#[async_trait]
impl Modifier for RandomDigitsModifier {

    fn name(&self) -> &'static str {
        "randomDigits"
    }

    async fn call(&self, ctx: Context) -> Context {
        let len = self.len.resolve(ctx).await;
        ctx.alter_value(Value::String(generate(len.as_usize().unwrap(), "1234567890")))
    }
}
