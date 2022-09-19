use async_trait::async_trait;
use cuid::slug;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct SlugModifier {}

impl SlugModifier {
    pub fn new() -> Self {
        SlugModifier {}
    }
}

#[async_trait]
impl Modifier for SlugModifier {

    fn name(&self) -> &'static str {
        "slug"
    }

    async fn call(&self, ctx: Context) -> Context {
        ctx.alter_value(Value::String(slug().unwrap()))
    }
}
