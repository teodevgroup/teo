use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Copy, Clone)]
pub struct AbsModifier {}

impl AbsModifier {
    pub fn new() -> Self {
        return AbsModifier {};
    }
}

#[async_trait]
impl Modifier for AbsModifier {

    fn name(&self) -> &'static str {
        "abs"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            if let Value::I8(i) = value {
                Stage::Value(Value::I8(i.abs()))
            } else if let Value::I16(i) = value {
                Stage::Value(Value::I16(i.abs()))
            } else if let Value::I32(i) = value {
                Stage::Value(Value::I32(i.abs()))
            } else if let Value::I64(i) = value {
                Stage::Value(Value::I64(i.abs()))
            } else if let Value::I128(i) = value {
                Stage::Value(Value::I128(i.abs()))
            } else if let Value::F32(f) = value {
                Stage::Value(Value::F32(f.abs()))
            } else if let Value::F64(f) = value {
                Stage::Value(Value::F64(f.abs()))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
