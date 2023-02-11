use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity::Invalid;
use crate::core::result::Result;
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
impl Item for IsHexColorModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    ctx
                } else {
                    ctx.with_validity(Invalid("String is not hex color.".to_owned()))
                }
            }
            None => {
                ctx.with_validity(Invalid("Value is not string.".to_owned()))
            }
        }
    }
}
