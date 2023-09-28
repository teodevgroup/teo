use async_trait::async_trait;

use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Clone)]
pub struct RegexReplaceItem {
    format: Value,
    substitute: Value,
}

impl RegexReplaceItem {
    pub fn new(format: impl Into<Value>, substitute: impl Into<Value>) -> Self {
        return RegexReplaceItem {
            format: format.into(),
            substitute: substitute.into(),
        };
    }
}

#[async_trait]
impl Item for RegexReplaceItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let arg = self.format.resolve(ctx.clone()).await?;
        let regex = arg.as_regexp().unwrap();
        let s_arg = self.substitute.resolve(ctx.clone()).await?;
        let substitute = s_arg.as_str().unwrap();
        match &ctx.get_value() {
            Value::String(s) => Ok(ctx.with_value(Value::String(regex.replace(s, substitute).to_string()))),
            _ => Err(ctx.internal_server_error("regexReplace: value is not string"))
        }
    }
}
