use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsAlphanumericModifier {}

impl IsAlphanumericModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsAlphanumericModifier {

    fn name(&self) -> &'static str {
        "isAlphanumeric"
    }

    async fn call(&self, context: Context) -> Context {
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
