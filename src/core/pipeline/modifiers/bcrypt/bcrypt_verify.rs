use async_trait::async_trait;
use bcrypt::verify;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::Validity::Invalid;

#[derive(Debug, Clone)]
pub struct BcryptVerifyModifier {
    argument: Argument
}

impl BcryptVerifyModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for BcryptVerifyModifier {

    fn name(&self) -> &'static str {
        "bcryptVerify"
    }

    async fn call(&self, context: Context) -> Context {
        match context.value.as_str() {
            None => {
                context.alter_validity(Invalid("Value is not string.".to_owned()))
            }
            Some(s) => {
                let hash = self.argument.resolve(context.clone()).await;
                match hash.as_str() {
                    None => context.alter_validity(Invalid("Hash argument is not string.".to_owned())),
                    Some(h) => {
                        if verify(s, h).unwrap() {
                            context.alter_value(Value::Null)
                        } else {
                            context.alter_validity(Invalid("Value is not correct.".to_owned()))
                        }
                    }
                }
            }
        }
    }
}
