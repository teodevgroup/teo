use async_trait::async_trait;
use chrono::Utc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Copy, Clone)]
pub struct NowModifier {}

impl NowModifier {
    pub fn new() -> Self {
        return NowModifier {};
    }
}

#[async_trait]
impl Modifier for NowModifier {

    fn name(&self) -> &'static str {
        "now"
    }

    async fn call(&self, _stage: Stage, _object: &Object) -> Stage {
        Stage::Value(Value::DateTime(Utc::now()))
    }
}
