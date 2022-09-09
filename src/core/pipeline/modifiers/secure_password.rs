use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Clone)]
pub struct SecurePasswordModifier {
    patterns: Vec<Regex>
}

impl SecurePasswordModifier {
    pub fn new() -> Self {
        return SecurePasswordModifier {
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
impl Modifier for SecurePasswordModifier {

    fn name(&self) -> &'static str {
        "secure_password"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                for regex in &self.patterns {
                    if !regex.is_match(&s) {
                        return Stage::Invalid(String::from("Value is not secure password."))
                    }
                }
                return Stage::Value(Value::String(s));
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
