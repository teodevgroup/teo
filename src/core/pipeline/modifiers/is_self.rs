use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct IsSelfModifier {}

impl IsSelfModifier {
    pub fn new() -> Self {
        return IsSelfModifier {};
    }
}

#[async_trait]
impl Modifier for IsSelfModifier {

    fn name(&self) -> &'static str {
        "is_self"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            if let Some(obj) = value.as_object() {
                if obj == object {
                    return stage.clone()
                } else {
                    return Stage::Invalid("Is not self.".to_string());
                }
            } else {
                return Stage::Invalid("Is not object or reference.".to_string());
            }
        } else {
            stage
        }
    }
}
