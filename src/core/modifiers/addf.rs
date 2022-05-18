use std::sync::{Arc};
use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct AddFModifier {
    addend: f64
}

impl AddFModifier {
    pub fn new(addend: f64) -> Self {
        return AddFModifier {
            addend
        };
    }
}

#[async_trait]
impl Modifier for AddFModifier {

    fn name(&self) -> &'static str {
        "addf"
    }

    async fn call(&self, stage: Stage, _object: Arc<Object>) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::F32(f) = value {
                Stage::Value(Value::F32(f + self.addend as f32))
            } else if let Value::F64(f) = value {
                Stage::Value(Value::F64(f + self.addend))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
