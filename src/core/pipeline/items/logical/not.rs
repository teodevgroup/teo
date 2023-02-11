use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct NotModifier {
    value: Value
}

impl NotModifier {
    pub fn new(value: Value) -> Self {
        return NotModifier {
            value
        };
    }
}

#[async_trait]
impl Item for NotModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match &self.value {
            Value::Pipeline(p) => if p.process(ctx.clone()).await.is_ok() {
                Err(ctx.with_invalid("value is not invalid"))
            } else {
                Ok(ctx)
            }
            _ => Err(ctx.internal_server_error("not: argument is not pipeline"))
        }
    }
}
