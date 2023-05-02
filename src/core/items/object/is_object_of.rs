use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct IsObjectOfItem {
    model: String
}

impl IsObjectOfItem {
    pub fn new(model: impl Into<String>) -> Self {
        IsObjectOfItem { model: model.into() }
    }
}

#[async_trait]
impl Item for IsObjectOfItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_object() {
            Some(o) => {
                if o.model().name() == &self.model {
                    Ok(ctx)
                } else {
                    let model = &self.model;
                    Err(ctx.with_invalid(format!("value is not object of {model}")))
                }
            }
            None => {
                if ctx.value.is_null() {
                    Err(ctx.with_invalid(format!("value is null")))
                } else {
                    Err(ctx.internal_server_error("isA: value is not object"))
                }
            }
        }
    }
}
