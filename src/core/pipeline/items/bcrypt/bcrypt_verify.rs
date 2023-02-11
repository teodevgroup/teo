use async_trait::async_trait;
use bcrypt::verify;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity::Invalid;
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
impl Item for BcryptVerifyModifier {

    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            None => {
                context.with_validity(Invalid("Value is not string.".to_owned()))
            }
            Some(s) => {
                let result = self.argument.process(context.clone()).await;
                if result.invalid_reason().is_some() {
                    return context.invalid(result.invalid_reason().unwrap());
                }
                let hash = result.value;
                match hash.as_str() {
                    None => context.with_validity(Invalid("Hash argument is not string.".to_owned())),
                    Some(h) => {
                        if verify(s, h).unwrap() {
                            context.with_value(Value::Null)
                        } else {
                            context.with_validity(Invalid("Value is not correct.".to_owned()))
                        }
                    }
                }
            }
        }
    }
}
