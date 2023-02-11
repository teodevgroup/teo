use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity::Invalid;

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
impl Item for IsEmailModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    context
                } else {
                    context.with_validity(Invalid("String is not email.".to_owned()))
                }
            }
            None => {
                context.with_validity(Invalid("Value is not string.".to_owned()))
            }
        }
    }
}
