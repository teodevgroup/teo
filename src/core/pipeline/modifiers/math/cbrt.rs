use num_integer::Roots;
use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct CbrtModifier { }

impl CbrtModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for CbrtModifier {

    fn name(&self) -> &'static str {
        "cbrt"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value {
            Value::I8(v) => ctx.alter_value(Value::I8(v.cbrt())),
            Value::I16(v) => ctx.alter_value(Value::I16(v.cbrt())),
            Value::I32(v) => ctx.alter_value(Value::I32(v.cbrt())),
            Value::I64(v) => ctx.alter_value(Value::I64(v.cbrt())),
            Value::I128(v) => ctx.alter_value(Value::I128(v.cbrt())),
            Value::U8(v) => ctx.alter_value(Value::U8(v.cbrt())),
            Value::U16(v) => ctx.alter_value(Value::U16(v.cbrt())),
            Value::U32(v) => ctx.alter_value(Value::U32(v.cbrt())),
            Value::U64(v) => ctx.alter_value(Value::U64(v.cbrt())),
            Value::U128(v) => ctx.alter_value(Value::U128(v.cbrt())),
            Value::F32(v) => ctx.alter_value(Value::F32(v.cbrt())),
            Value::F64(v) => ctx.alter_value(Value::F64(v.cbrt())),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
