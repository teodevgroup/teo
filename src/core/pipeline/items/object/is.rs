use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct IsItem {
    value: Value
}

impl IsItem {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[async_trait]
impl Item for IsItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_ref().as_object() {
            Some(o) => {
                let rhs = self.value.resolve(ctx.clone()).await?;
                if !rhs.is_object() {
                    return Err(ctx.internal_server_error("is: argument 0 is not object"))
                }
                let rhs = rhs.as_object().unwrap();
                if o.model().name() != rhs.model().name() {
                    return Err(ctx.with_invalid("value object is not argument object"))
                }
                if o.identifier() != rhs.identifier() {
                    return Err(ctx.with_invalid("value object is not argument object"))
                }
                Ok(ctx)
            }
            None => {
                if ctx.value.as_ref().is_null() {
                    Err(ctx.with_invalid("value object is null"))
                } else {
                    Err(ctx.internal_server_error("is: value is not object"))
                }
            }
        }
    }
}
