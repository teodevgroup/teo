use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct PowModifier {
    argument: Value
}

impl PowModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for PowModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        let exp = argument.as_i32().unwrap() as u32;
        match ctx.value {
            Value::I32(v) => ctx.with_value(Value::I32(v.pow(exp))),
            Value::I64(v) => ctx.with_value(Value::I64(v.pow(exp))),
            Value::F32(v) => ctx.with_value(Value::F32(v.powf(argument.as_f32().unwrap()))),
            Value::F64(v) => ctx.with_value(Value::F64(v.powf(argument.as_f64().unwrap()))),
            _ => ctx.invalid("Value is not number."),
        }
    }
}
