use async_trait::async_trait;
use cuid::cuid;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct CUIDModifier {}

impl CUIDModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for CUIDModifier {

    fn name(&self) -> &'static str {
        "cuid"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        context.alter_value(Value::String(cuid().unwrap()))
    }
}
