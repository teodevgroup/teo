use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsNullModifier {}

impl IsNullModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsNullModifier {

    fn name(&self) -> &'static str {
        "is_null"
    }

    async fn call(&self, ctx: Context) -> Context {
        if ctx.value.is_null() {
            ctx
        } else {
            ctx.invalid("Value is not null.")
        }
    }
}
