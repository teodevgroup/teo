use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash};
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct BcryptSaltItem { }

impl BcryptSaltItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for BcryptSaltItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
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
