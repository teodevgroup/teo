use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct SelfItem { }

impl SelfItem {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for SelfItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.object.as_ref() {
            Some(obj) => Ok(ctx.with_value(Value::Object(obj.clone()))),
            None => Err(ctx.internal_server_error("self: ctx object does not exist"))
        }
    }
}
