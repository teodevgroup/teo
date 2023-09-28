use async_trait::async_trait;
use crate::core::initiator::Initiator;
use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::pipeline::Pipeline;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct IdentityItem {
    pipeline: Pipeline
}

impl IdentityItem {
    pub fn new(pipeline: Pipeline) -> Self {
        Self { pipeline }
    }
}

#[async_trait]
impl Item for IdentityItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.get_object()?.action_source() {
            Initiator::Identity(user, _) => {
                let user = match user {
                    Some(u) => Value::Object(u.clone()),
                    None => Value::Null,
                };
                Ok(ctx.with_value(self.pipeline.process(ctx.with_value(user)).await?))
            }
            _ => Ok(ctx)
        }
    }
}
