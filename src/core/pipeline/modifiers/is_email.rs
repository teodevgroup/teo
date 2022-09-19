use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


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

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                if self.regex.is_match(&s) {
                    Stage::Value(Value::String(s))
                } else {
                    Stage::Invalid(String::from("Invalid email."))
                }
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
