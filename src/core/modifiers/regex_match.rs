use async_trait::async_trait;
use regex::Regex;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Clone)]
pub struct RegexMatchModifier {
    regex: Regex
}

impl RegexMatchModifier {
    pub fn new(format: &'static str) -> Self {
        return RegexMatchModifier {
            regex: Regex::new(format).unwrap()
        };
    }
}

#[async_trait]
impl Modifier for RegexMatchModifier {

    fn name(&self) -> &'static str {
        "regex_match"
    }

    async fn call(&self, stage: Stage, _object: Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                if self.regex.is_match(&s) {
                    Stage::Value(Value::String(s))
                } else {
                    Stage::Invalid(String::from("String format doesn't match."))
                }
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
