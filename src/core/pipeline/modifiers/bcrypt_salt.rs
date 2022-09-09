use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct BcryptSaltModifier {}

impl BcryptSaltModifier {
    pub fn new() -> Self {
        return BcryptSaltModifier {};
    }
}

#[async_trait]
impl Modifier for BcryptSaltModifier {

    fn name(&self) -> &'static str {
        "bcrypt_salt"
    }

    async fn call(&self, stage: Stage, _object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                let hashed = hash(s.as_str(), DEFAULT_COST).unwrap();
                Stage::Value(Value::String(hashed))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
