use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity::Invalid;

#[derive(Debug, Copy, Clone)]
pub struct BcryptSaltModifier {}

impl BcryptSaltModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for BcryptSaltModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.value.as_str() {
            Some(s) => {
                context.with_value(Value::String(hash(s, DEFAULT_COST).unwrap()))
            }
            None => {
                context.with_validity(Invalid("Value is not string.".to_owned()))
            }
        }
    }
}
