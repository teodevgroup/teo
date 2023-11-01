use async_trait::async_trait;
use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Clone)]
pub struct TruncateItem {
    argument: Value,
}

impl TruncateItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self {
            argument: argument.into(),
        }
    }
}

#[async_trait]
impl Item for TruncateItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?.as_usize().unwrap();
        match &ctx.value {
            Value::String(s) => Ok(ctx.with_value(Value::String(s.chars().take(argument).collect()))),
            Value::Vec(v) => Ok(ctx.with_value(Value::Vec(v.iter().take(argument).map(|v| v.clone()).collect()))),
            _ => Err(ctx.internal_server_error("truncate: value is not vector"))
        }
    }
}