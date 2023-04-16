use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Copy, Clone)]
pub struct GetLengthItem {}

impl GetLengthItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for GetLengthItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let len = match &ctx.value {
            Value::String(s) => s.len(),
            Value::Vec(v) => v.len(),
            _ => {
                return Err(ctx.internal_server_error("getLength: value is not vector"));
            }
        };
        Ok(ctx.with_value(Value::I32(len as i32)))
    }
}
