use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct IsFalseModifier {}

impl IsFalseModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsFalseModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
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
