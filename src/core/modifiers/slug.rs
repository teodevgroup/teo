use async_trait::async_trait;
use cuid::slug;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct SlugModifier {}

impl SlugModifier {
    pub fn new() -> Self {
        return SlugModifier {};
    }
}

#[async_trait]
impl Modifier for SlugModifier {

    fn name(&self) -> &'static str {
        "slug"
    }

    async fn call(&self, _stage: Stage, _object: &Object) -> Stage {
        Stage::Value(Value::String(slug().unwrap()))
    }
}
