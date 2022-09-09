use async_trait::async_trait;
use crate::core::argument::Argument;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Clone)]
pub struct LengthModifier {
    len: Argument
}

impl LengthModifier {
    pub fn new(len: impl Into<Argument>) -> Self {
        return LengthModifier { len: len.into() };
    }
}


#[async_trait]
impl Modifier for LengthModifier {

    fn name(&self) -> &'static str {
        "length"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        let len = self.len.resolve(stage.clone(), object).await.as_usize().unwrap();
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                if s.len() == len {
                    Stage::Value(Value::String(s))
                } else {
                    Stage::Invalid("Bad length.".to_string())
                }
            } else if let Value::Vec(v) = value {
                if v.len() == len {
                    Stage::Value(Value::Vec(v))
                } else {
                    Stage::Invalid("Bad length.".to_string())
                }
            } else {
                stage
            }
        } else {
            stage
        }
    }
}
