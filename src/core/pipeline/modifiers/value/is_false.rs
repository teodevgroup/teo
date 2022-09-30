use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsFalseModifier {}

impl IsFalseModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsFalseModifier {

    fn name(&self) -> &'static str {
        "isFalse"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let valid = match ctx.value.as_bool() {
            Some(b) => !b,
            None => false
        };
        if valid {
            ctx
        } else {
            ctx.invalid("Value is not false.")
        }
    }
}
