use async_trait::async_trait;
use crate::core::argument::Argument;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Clone)]
pub struct LengthBetweenModifier {
    min: Argument,
    max: Argument,
}

impl LengthBetweenModifier {
    pub fn new(min: impl Into<Argument>, max: impl Into<Argument>) -> Self {
        return LengthBetweenModifier { min: min.into(), max: max.into() };
    }
}


#[async_trait]
impl Modifier for LengthBetweenModifier {

    fn name(&self) -> &'static str {
        "length_between"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        let min = self.min.resolve(stage.clone(), object).await.as_usize().unwrap();
        let max = self.max.resolve(stage.clone(), object).await.as_usize().unwrap();
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                if min <= s.len() && s.len() <= max {
                    Stage::Value(Value::String(s))
                } else {
                    Stage::Invalid("Bad length.".to_string())
                }
            } else if let Value::Vec(v) = value {
                if min <= v.len() && v.len() <= max {
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
