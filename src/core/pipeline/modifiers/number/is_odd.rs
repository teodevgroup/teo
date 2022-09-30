use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct IsOddModifier {}

impl IsOddModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsOddModifier {

    fn name(&self) -> &'static str {
        "isOdd"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::I8(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::I16(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::I32(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::I64(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::I128(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::U8(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::U16(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::U32(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::U64(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::U128(v) => if v % 2 == 1 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::F32(v) => if v % 2.0 == 1.0 { ctx } else { ctx.invalid("Value is not odd.") },
            Value::F64(v) => if v % 2.0 == 1.0 { ctx } else { ctx.invalid("Value is not odd.") },
            _ => ctx.invalid("Value is not number.")
        }
    }
}
