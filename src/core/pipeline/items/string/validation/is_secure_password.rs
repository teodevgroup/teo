use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct IsSecurePasswordModifier {
    patterns: Vec<Regex>
}

impl IsSecurePasswordModifier {
    pub fn new() -> Self {
        return IsSecurePasswordModifier {
            patterns: vec![
                Regex::new(r#"[A-Z]"#).unwrap(),
                Regex::new(r#"[a-z]"#).unwrap(),
                Regex::new(r#"\d"#).unwrap(),
                Regex::new(r#"[!@#$&*`~()\-_+=\[\]{}:;'",<>.?\\|/]"#).unwrap(),
            ]
        };
    }
}

#[async_trait]
impl Item for IsSecurePasswordModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                for regex in &self.patterns {
                    if !regex.is_match(&s) {
                        return Err(ctx.invalid("value is not secure password"));
                    }
                }
                Ok(ctx)
            }
            None => Err(ctx.internal_server_error("isSecurePassword: value is not string"))
        }
    }
}
