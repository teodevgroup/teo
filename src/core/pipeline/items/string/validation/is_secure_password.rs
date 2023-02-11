use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;

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
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            Some(s) => {
                for regex in &self.patterns {
                    if !regex.is_match(&s) {
                        return context.invalid("Value is not secure password.");
                    }
                }
                context
            }
            None => context.invalid("Value is not string.")
        }
    }
}
