use std::cmp::{max};
use async_trait::async_trait;
use crate::core::teon::Value;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct MinModifier {
    argument: Value
}

impl MinModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for MinModifier {

    fn name(&self) -> &'static str {
        "min"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match ctx.value {
            Value::I32(v) => ctx.alter_value(Value::I32(max(v, argument.as_i32().unwrap()))),
            Value::I64(v) => ctx.alter_value(Value::I64(max(v, argument.as_i64().unwrap()))),
            Value::F32(v) => ctx.alter_value(Value::F32(if v >= argument.as_f32().unwrap() { v } else { argument.as_f32().unwrap() })),
            Value::F64(v) => ctx.alter_value(Value::F64(if v >= argument.as_f64().unwrap() { v } else { argument.as_f64().unwrap() })),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
