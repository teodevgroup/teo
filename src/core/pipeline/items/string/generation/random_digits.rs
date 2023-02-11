use async_trait::async_trait;
use random_string::generate;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

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
impl Item for RandomDigitsModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let len = self.len.resolve(ctx.clone()).await;
        ctx.with_value(Value::String(generate(len.as_usize().unwrap(), "1234567890")))
    }
}
