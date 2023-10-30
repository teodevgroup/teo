use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::prelude::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct JoinItem {
    separator: Value
}

impl JoinItem {
    pub fn new(separator: Value) -> Self {
        Self { separator }
    }
}

#[async_trait]
impl Item for JoinItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_vec() {
            None => Err(ctx.internal_server_error("join: value is not vector")),
            Some(v) => {
                let arg = self.separator.resolve(ctx.clone()).await?;
                let separator = arg.as_str().unwrap();
                Ok(ctx.with_value(Value::String(v.iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>().join(separator))))
            }
        }
    }
}
