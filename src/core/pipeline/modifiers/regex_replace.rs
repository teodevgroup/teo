use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Clone)]
pub struct RegexReplaceModifier {
    regex: Regex,
    substitute: &'static str
}

impl RegexReplaceModifier {
    pub fn new(format: &'static str, substitute: &'static str) -> Self {
        return RegexReplaceModifier {
            regex: Regex::new(format).unwrap(),
            substitute
        };
    }
}

#[async_trait]
impl Modifier for RegexReplaceModifier {

    fn name(&self) -> &'static str {
        "regex_replace"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                Stage::Value(Value::String(self.regex.replace(&s, self.substitute).to_string()))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
