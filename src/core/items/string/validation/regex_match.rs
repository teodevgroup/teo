use async_trait::async_trait;
use crate::core::result::Result;
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Clone)]
pub struct RegexMatchItem {
    argument: Value
}

impl RegexMatchItem {
    pub fn new(format: impl Into<Value>) -> Self {
        Self {
            argument: format.into()
        }
    }
}

#[async_trait]
impl Item for RegexMatchItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let arg_value = self.argument.resolve(ctx.clone()).await?;
        let regex = arg_value.as_regexp().unwrap();
        match ctx.get_value() {
            Value::String(s) => {
                if regex.is_match(&s) {
                    Ok(ctx.clone())
                } else {
                    Err(ctx.internal_server_error("value does not match regex"))
                }
            }
            _ => {
                Err(ctx.internal_server_error("regexMatch: value is not string"))
            }
        }
    }
}
