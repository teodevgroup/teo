use async_trait::async_trait;

use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;
use inflector::Inflector;

#[derive(Debug, Copy, Clone)]
pub struct ToTitleCaseItem {}

impl ToTitleCaseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ToTitleCaseItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.get_value() {
            Value::String(s) => Ok(ctx.with_value(Value::String(s.to_title_case()))),
            _ => Err(ctx.internal_server_error("value is not string"))
        }
    }
}
