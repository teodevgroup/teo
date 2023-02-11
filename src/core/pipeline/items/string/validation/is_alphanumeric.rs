use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct IsAlphanumericModifier {}

impl IsAlphanumericModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsAlphanumericModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            Some(s) => {
                for c in s.chars() {
                    if !c.is_alphanumeric() {
                        return context.invalid("Value is not alphanumeric.");
                    }
                }
                context
            }
            None => {
                context.invalid("Value is not string.")
            }
        }
    }
}
