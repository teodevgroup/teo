use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Validity::Invalid;

#[derive(Debug, Clone)]
pub struct IsEmailModifier {
    regex: Regex
}

impl IsEmailModifier {
    pub fn new() -> Self {
        return IsEmailModifier {
            regex: Regex::new(r"^\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b$").unwrap()
        };
    }
}

#[async_trait]
impl Modifier for IsEmailModifier {

    fn name(&self) -> &'static str {
        "isEmail"
    }

    async fn call(&self, context: Context) -> Context {
        match context.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    context
                } else {
                    context.alter_validity(Invalid("String is not email.".to_owned()))
                }
            }
            None => {
                context.alter_validity(Invalid("Value is not string.".to_owned()))
            }
        }
    }
}
