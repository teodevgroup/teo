use async_trait::async_trait;
use pad::{PadStr, Alignment};
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct PadStartModifier {
    char: char,
    width: Value,
}

impl PadStartModifier {
    pub fn new(char: char, width: impl Into<Value>) -> Self {
        Self { char, width: width.into() }
    }
}

#[async_trait]
impl Item for PadStartModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => Err(ctx.internal_server_error("padStart: value is not string")),
            Some(s) => {
                let arg = self.width.resolve(ctx.clone()).await?;
                let width = arg.as_i64().unwrap() as usize;
                let char = self.char;
                Ok(ctx.with_value(Value::String(s.pad(width, char, Alignment::Left, false))))
            }
        }
    }
}
