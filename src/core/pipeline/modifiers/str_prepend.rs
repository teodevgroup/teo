use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Clone)]
pub struct StrPrependModifier {
    prefix: &'static str
}

impl StrPrependModifier {
    pub fn new(prefix: &'static str) -> Self {
        return StrPrependModifier {
            prefix
        };
    }
}

#[async_trait]
impl Modifier for StrPrependModifier {

    fn name(&self) -> &'static str {
        "str_prepend"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                let mut r = String::new();
                r.push_str(&self.prefix);
                r.push_str(&s);
                Stage::Value(Value::String(r))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }

    }
}
