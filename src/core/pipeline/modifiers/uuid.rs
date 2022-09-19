use async_trait::async_trait;
use uuid::Uuid;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Copy, Clone)]
pub struct UUIDModifier {}

impl UUIDModifier {
    pub fn new() -> Self {
        return UUIDModifier {};
    }
}

#[async_trait]
impl Modifier for UUIDModifier {

    fn name(&self) -> &'static str {
        "uuid"
    }

    async fn call(&self, _stage: Stage, _object: &Object) -> Stage {
        Stage::Value(Value::String(Uuid::new_v4().to_string()))
    }
}
