use async_trait::async_trait;
use cuid::cuid;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct CUIDModifier {}

impl CUIDModifier {
    pub fn new() -> Self {
        return CUIDModifier {};
    }
}

#[async_trait]
impl Modifier for CUIDModifier {

    fn name(&self) -> &'static str {
        "cuid"
    }

    async fn call(&self, _stage: Stage, _object: &Object) -> Stage {
        Stage::Value(Value::String(cuid().unwrap()))
    }
}
