use async_trait::async_trait;
use pad::{PadStr, Alignment};
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct PadStartModifier {
    char: char,
    width: Argument,
}

impl PadStartModifier {
    pub fn new(char: char, width: impl Into<Argument>) -> Self {
        Self { char, width: width.into() }
    }
}

#[async_trait]
impl Modifier for PadStartModifier {

    fn name(&self) -> &'static str {
        "padStart"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.width.resolve(ctx.clone()).await;
                let width = arg.as_u32().unwrap() as usize;
                let char = self.char;
                ctx.alter_value(Value::String(s.pad(width, char, Alignment::Left, false)))
            }
        }
    }
}
