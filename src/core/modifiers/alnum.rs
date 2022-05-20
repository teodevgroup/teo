use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct AlnumModifier {}

impl AlnumModifier {
    pub fn new() -> Self {
        return AlnumModifier {};
    }
}

#[async_trait]
impl Modifier for AlnumModifier {

    fn name(&self) -> &'static str {
        "alnum"
    }

    async fn call(&self, stage: Stage, _object: Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                for c in s.chars() {
                    if !c.is_alphanumeric() {
                        return Stage::Invalid(String::from("Invalid alnum string."))
                    }
                }
                Stage::Value(Value::String(s))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
