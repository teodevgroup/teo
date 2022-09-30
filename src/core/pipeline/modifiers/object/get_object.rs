use async_trait::async_trait;
use key_path::path;

use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub struct GetObjectModifier { }

impl GetObjectModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for GetObjectModifier {

    fn name(&self) -> &'static str {
        "getObject"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        ctx.alter_value(Value::Object(ctx.object.clone())).alter_key_path(path![])
    }
}
