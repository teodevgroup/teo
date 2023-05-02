use async_trait::async_trait;

use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;
use inflector::cases::wordcase::to_word_case;

#[derive(Debug, Copy, Clone)]
pub struct ToWordCaseItem {}

impl ToWordCaseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ToWordCaseItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.get_value() {
            Value::String(ref s) => Ok(ctx.with_value(Value::String(to_word_case(s)))),
            _ => Err(ctx.internal_server_error("value is not string"))
        }
    }
}
