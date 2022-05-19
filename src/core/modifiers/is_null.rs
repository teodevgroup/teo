use std::sync::{Arc};
use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct IsNullModifier {}

impl IsNullModifier {
    pub fn new() -> Self {
        return IsNullModifier {};
    }
}

#[async_trait]
impl Modifier for IsNullModifier {

    fn name(&self) -> &'static str {
        "is_null"
    }

    async fn call(&self, stage: Stage, _object: Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::Null = value {
                Stage::Value(Value::Null)
            } else {
                Stage::Invalid(String::from("Value should be null."))
            }
        } else {
            stage
        }

    }
}
