use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct BoolModifier {}

impl BoolModifier {
    pub fn new() -> Self {
        return BoolModifier {};
    }
}

#[async_trait]
impl Modifier for BoolModifier {

    fn name(&self) -> &'static str {
        "bool"
    }

    async fn call(&self, stage: Stage, _object: Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::Null = value {
                Stage::Value(Value::Null)
            } else if let Value::Bool(b) = value {
                Stage::Value(Value::Bool(b))
            } else {
                Stage::Invalid(String::from("Value should be bool."))
            }
        } else {
            stage
        }

    }
}
