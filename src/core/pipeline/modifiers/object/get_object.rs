use async_trait::async_trait;
use crate::core::key_path::KeyPathItem;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::argument::Argument;
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

    async fn call(&self, ctx: Context) -> Context {
        ctx.alter_value(Value::Object(ctx.object.clone())).alter_key_path(vec![])
    }
}
