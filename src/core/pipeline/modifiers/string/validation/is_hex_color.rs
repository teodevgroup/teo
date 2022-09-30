use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Validity::Invalid;

#[derive(Debug, Clone)]
pub struct IsHexColorModifier {
    regex: Regex
}

impl IsHexColorModifier {
    pub fn new() -> Self {
        return IsHexColorModifier {
            regex: Regex::new(r"^[A-Fa-f0-9]{6}$").unwrap()
        };
    }
}

#[async_trait]
impl Modifier for IsHexColorModifier {

    fn name(&self) -> &'static str {
        "isHexColor"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        match context.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    context
                } else {
                    context.alter_validity(Invalid("String is not hex color.".to_owned()))
                }
            }
            None => {
                context.alter_validity(Invalid("Value is not string.".to_owned()))
            }
        }
    }
}
