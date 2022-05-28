use async_trait::async_trait;
use random_string::generate;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct RandomDigitsModifier {
    len: usize
}

impl RandomDigitsModifier {
    pub fn new(len: usize) -> Self {
        return RandomDigitsModifier {
            len
        };
    }
}

#[async_trait]
impl Modifier for RandomDigitsModifier {

    fn name(&self) -> &'static str {
        "random_digits"
    }

    async fn call(&self, _stage: Stage, _object: &Object) -> Stage {
        return Stage::Value(Value::String(generate(self.len, "1234567890")))
    }
}
