use async_trait::async_trait;
use key_path::path;

use crate::core::pipeline::item::Item;
use crate::core::result::Result;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct GetObjectModifier { }

impl GetObjectModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Item for GetObjectModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        ctx.with_value(Value::Object(ctx.object.as_ref().unwrap().clone())).with_path(path![])
    }
}
