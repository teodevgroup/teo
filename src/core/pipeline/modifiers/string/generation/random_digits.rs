use async_trait::async_trait;
use random_string::generate;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct RandomDigitsModifier {
    len: Value
}

impl RandomDigitsModifier {
    pub fn new(len: impl Into<Value>) -> Self {
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

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let len = self.len.resolve(ctx.clone()).await;
        ctx.alter_value(Value::String(generate(len.as_usize().unwrap(), "1234567890")))
    }
}
