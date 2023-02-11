use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
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
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                Ok(ctx.with_value(Value::String(hash(s, DEFAULT_COST).unwrap())))
            }
            None => {
                Err(ctx.internal_server_error("bcryptSalt: value is not string"))
            }
        }
    }
}
