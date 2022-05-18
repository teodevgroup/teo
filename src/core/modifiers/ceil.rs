use std::sync::{Arc};
use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct CeilModifier {}

impl CeilModifier {
    pub fn new() -> Self {
        return CeilModifier {};
    }
}

#[async_trait]
impl Modifier for CeilModifier {

    fn name(&self) -> &'static str {
        "ceil"
    }

    async fn call(&self, stage: Stage, _object: Arc<Object>) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::F32(f) = value {
                Stage::Value(Value::F32(f.ceil()))
            } else if let Value::F64(f) = value {
                Stage::Value(Value::F64(f.ceil()))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }

    }
}
