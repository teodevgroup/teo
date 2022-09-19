use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;


#[derive(Debug, Copy, Clone)]
pub struct IsInstanceOfModifier {
    model_name: &'static str
}

impl IsInstanceOfModifier {
    pub fn new(model_name: &'static str) -> Self {
        return IsInstanceOfModifier { model_name };
    }
}

#[async_trait]
impl Modifier for IsInstanceOfModifier {

    fn name(&self) -> &'static str {
        "is_instance_of"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            if let Some(obj) = value.as_object() {
                if obj.is_instance_of(self.model_name) {
                    return stage.clone()
                } else {
                    let model_name = self.model_name;
                    return Stage::Invalid(format!("Is not instance of '{model_name}'."));
                }
            } else {
                return Stage::Invalid("Is not object or reference.".to_string());
            }
        } else {
            stage
        }
    }
}
