use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct AssignItem {
    key: Value,
    value: Value,
}

impl AssignItem {
    pub fn new(key: Value, value: Value) -> Self {
        Self {
            key,
            value,
        }
    }
}

#[async_trait]
impl Item for AssignItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.object.as_ref() {
            None => Err(ctx.internal_server_error("assign: ctx object does not exist")),
            Some(obj) => {
                let key = self.key.resolve(ctx.clone()).await?;
                let key_str = key.as_raw_enum_choice().unwrap();
                let value = self.value.resolve(ctx.clone()).await?;
                obj.set_value(key_str, value)?;
                Ok(ctx)
            },
        }
    }
}
