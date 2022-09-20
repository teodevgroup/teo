use async_trait::async_trait;
use cuid::slug;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct TrimModifier {}

impl TrimModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for TrimModifier {

    fn name(&self) -> &'static str {
        "trim"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value {
            Value::String(ref s) => ctx.alter_value(Value::String(s.trim().to_owned())),
            _ => ctx.invalid("Value is not string.")
        }
    }
}
