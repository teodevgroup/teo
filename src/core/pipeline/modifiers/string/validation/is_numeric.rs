use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsNumericModifier {}

impl IsNumericModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsNumericModifier {

    fn name(&self) -> &'static str {
        "isNumeric"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
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
