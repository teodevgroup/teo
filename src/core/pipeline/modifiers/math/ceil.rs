use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct CeilModifier {}

impl CeilModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for CeilModifier {

    fn name(&self) -> &'static str {
        "ceil"
    }

    async fn call(&self, context: Context) -> Context {
        match context.value {
            Value::F32(v) => context.alter_value(Value::F32(v.ceil())),
            Value::F64(v) => context.alter_value(Value::F64(v.ceil())),
            _ => context
        }
    }
}
