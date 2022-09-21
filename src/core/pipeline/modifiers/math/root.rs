use async_trait::async_trait;
use num_integer::Roots;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct RootModifier {
    argument: Argument
}

impl RootModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for RootModifier {

    fn name(&self) -> &'static str {
        "root"
    }

    async fn call(&self, ctx: Context) -> Context {
        let argument = self.argument.resolve(ctx.clone()).await;
        let exp = argument.as_u32().unwrap();
        match ctx.value {
            Value::I8(v) => ctx.alter_value(Value::I8(v.nth_root(exp))),
            Value::I16(v) => ctx.alter_value(Value::I16(v.nth_root(exp))),
            Value::I32(v) => ctx.alter_value(Value::I32(v.nth_root(exp))),
            Value::I64(v) => ctx.alter_value(Value::I64(v.nth_root(exp))),
            Value::I128(v) => ctx.alter_value(Value::I128(v.nth_root(exp))),
            Value::U8(v) => ctx.alter_value(Value::U8(v.nth_root(exp))),
            Value::U16(v) => ctx.alter_value(Value::U16(v.nth_root(exp))),
            Value::U32(v) => ctx.alter_value(Value::U32(v.nth_root(exp))),
            Value::U64(v) => ctx.alter_value(Value::U64(v.nth_root(exp))),
            Value::U128(v) => ctx.alter_value(Value::U128(v.nth_root(exp))),
            Value::F32(_v) => ctx.invalid("F32 value doesn't support nth root yet."),
            Value::F64(_v) => ctx.invalid("F64 value doesn't support nth root yet."),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
