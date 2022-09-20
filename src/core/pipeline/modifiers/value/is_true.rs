use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsTrueModifier {}

impl IsTrueModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsTrueModifier {

    fn name(&self) -> &'static str {
        "isTrue"
    }

    async fn call(&self, ctx: Context) -> Context {
        let valid = match ctx.value.as_bool() {
            Some(b) => b,
            None => false
        };
        if valid {
            ctx
        } else {
            ctx.invalid("Value is not true.")
        }
    }
}
