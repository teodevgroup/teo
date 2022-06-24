use async_trait::async_trait;
use bcrypt::verify;
use crate::core::argument::Argument;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Clone)]
pub struct BcryptVerifyModifier {
    argument: Argument
}

impl BcryptVerifyModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        return BcryptVerifyModifier { argument: argument.into() };
    }
}

#[async_trait]
impl Modifier for BcryptVerifyModifier {

    fn name(&self) -> &'static str {
        "bcrypt_verify"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::String(s) = value {
                let correct = verify(s, self.argument.resolve(stage, object).await.as_str().unwrap()).unwrap();
                if correct {
                    Stage::Value(Value::Null)
                } else {
                    Stage::Invalid("Password is not correct.".to_string())
                }
            } else {
                Stage::Invalid("Wrong value type.".to_string())
            }
        } else {
            Stage::Invalid("Wrong value type.".to_string())
        }
    }
}
