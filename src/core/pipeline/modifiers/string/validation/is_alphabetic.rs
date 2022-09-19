use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsAlphabeticModifier {}

impl IsAlphabeticModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsAlphabeticModifier {

    fn name(&self) -> &'static str {
        "isAlphabetic"
    }

    async fn call(&self, context: Context) -> Context {
        match context.value.as_str() {
            None => context.invalid("Value is not string."),
            Some(s) => {
                for c in s.chars() {
                    if !c.is_alphabetic() {
                        context.invalid("Value is not alphabetic.")
                    }
                }
                context
            }
        }
    }
}
