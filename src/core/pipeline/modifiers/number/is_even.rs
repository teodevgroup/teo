use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct IsEvenModifier {}

impl IsEvenModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsEvenModifier {

    fn name(&self) -> &'static str {
        "isEven"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value {
            Value::I8(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::I16(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::I32(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::I64(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::I128(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::U8(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::U16(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::U32(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::U64(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::U128(v) => if v % 2 == 0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::F32(v) => if v % 2.0 == 0.0 { ctx } else { ctx.invalid("Value is not even.") },
            Value::F64(v) => if v % 2.0 == 0.0 { ctx } else { ctx.invalid("Value is not even.") },
            _ => ctx.invalid("Value is not number.")
        }
    }
}
