use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct AlphaModifier {}

impl AlphaModifier {
    pub fn new() -> Self {
        return AlphaModifier {};
    }
}

#[async_trait]
impl Modifier for AlphaModifier {

    fn name(&self) -> &'static str {
        "alpha"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                for c in s.chars() {
                    if !c.is_alphabetic() {
                        return Stage::Invalid(String::from("Invalid alpha string."))
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
