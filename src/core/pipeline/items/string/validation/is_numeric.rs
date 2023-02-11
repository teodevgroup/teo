use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct IsNumericModifier {}

impl IsNumericModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for IsNumericModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            Some(s) => {
                for c in s.chars() {
                    if !c.is_numeric() {
                        return context.invalid("Value is not numeric.");
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
