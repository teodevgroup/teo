use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct AbsModifier {}

impl AbsModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for AbsModifier {

    fn name(&self) -> &'static str {
        "abs"
    }

    async fn call(&self, context: Context) -> Context {
        match context.value {
            Value::I8(v) => context.alter_value(Value::I8(v.abs())),
            Value::I16(v) => context.alter_value(Value::I16(v.abs())),
            Value::I32(v) => context.alter_value(Value::I32(v.abs())),
            Value::I64(v) => context.alter_value(Value::I64(v.abs())),
            Value::I128(v) => context.alter_value(Value::I128(v.abs())),
            Value::F32(v) => context.alter_value(Value::F32(v.abs())),
            Value::F64(v) => context.alter_value(Value::F64(v.abs())),
            _ => context
        }
    }
}
