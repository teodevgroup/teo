use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Clone)]
pub struct StrAppendModifier {
    suffix: &'static str
}

impl StrAppendModifier {
    pub fn new(suffix: &'static str) -> Self {
        return StrAppendModifier {
            suffix
        };
    }
}

#[async_trait]
impl Modifier for StrAppendModifier {

    fn name(&self) -> &'static str {
        "str_append"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                let mut r = String::new();
                r.push_str(&s);
                r.push_str(&self.suffix);
                Stage::Value(Value::String(r))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
