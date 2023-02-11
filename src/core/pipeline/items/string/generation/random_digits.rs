use async_trait::async_trait;
use random_string::generate;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct RandomDigitsItem {
    len: Value
}

impl RandomDigitsItem {
    pub fn new(len: impl Into<Value>) -> Self {
        return RandomDigitsItem {
            len: len.into()
        };
    }
}

#[async_trait]
impl Item for RandomDigitsItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let len = self.len.resolve(ctx.clone()).await?;
        Ok(ctx.with_value(Value::String(generate(len.as_usize().unwrap(), "1234567890"))))
    }
}
