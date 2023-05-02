use async_trait::async_trait;
use regex::Regex;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct IsHexColorItem {
    regex: Regex
}

impl IsHexColorItem {
    pub fn new() -> Self {
        return IsHexColorItem {
            regex: Regex::new(r"^[A-Fa-f0-9]{6}$").unwrap()
        };
    }
}

#[async_trait]
impl Item for IsHexColorItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                if self.regex.is_match(s) {
                    Ok(ctx)
                } else {
                    Err(ctx.with_invalid("String is not hex color."))
                }
            }
            None => {
                Err(ctx.internal_server_error("isHexColor: value is not string"))
            }
        }
    }
}
