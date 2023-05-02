use async_trait::async_trait;
use key_path::path;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct GetPreviousItem {
    key: Value
}

impl GetPreviousItem {
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl Item for GetPreviousItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let key = self.key.resolve(ctx.clone()).await?;
        let value = ctx.object.as_ref().unwrap().get_previous_value(key.str_from_string_or_raw_enum_choice().unwrap()).unwrap();
        Ok(ctx.with_value(value).with_path(path![key.as_str().unwrap().to_string()]))
    }
}
