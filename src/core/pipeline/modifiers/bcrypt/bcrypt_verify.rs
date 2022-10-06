use async_trait::async_trait;
use bcrypt::verify;

use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;

use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::validity::Validity::Invalid;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct BcryptVerifyModifier {
    argument: Pipeline
}

impl BcryptVerifyModifier {
    pub fn new(argument: Pipeline) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Modifier for BcryptVerifyModifier {

    fn name(&self) -> &'static str {
        "bcryptVerify"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        match context.value.as_str() {
            None => {
                context.alter_validity(Invalid("Value is not string.".to_owned()))
            }
            Some(s) => {
                let result = self.argument.process(context.clone()).await;
                if result.invalid_reason().is_some() {
                    return context.invalid(result.invalid_reason().unwrap());
                }
                let hash = result.value;
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
