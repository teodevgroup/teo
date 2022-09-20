use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

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
impl Modifier for IsSecurePasswordModifier {

    fn name(&self) -> &'static str {
        "isSecurePassword"
    }

    async fn call(&self, context: Context) -> Context {
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
