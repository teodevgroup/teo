use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct FloorModifier {}

impl FloorModifier {
    pub fn new() -> Self {
        return FloorModifier {};
    }
}

#[async_trait]
impl Modifier for FloorModifier {

    fn name(&self) -> &'static str {
        "floor"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::F32(f) = value {
                Stage::Value(Value::F32(f.floor()))
            } else if let Value::F64(f) = value {
                Stage::Value(Value::F64(f.floor()))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }

    }
}
