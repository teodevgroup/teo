use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct AllModifier {
    pipelines: Vec<Pipeline>
}

impl AllModifier {
    pub fn new(pipelines: Vec<Pipeline>) -> Self {
        Self { pipelines }
    }
}

#[async_trait]
impl Item for AllModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        for pipeline in &self.pipelines {
            pipeline.process(ctx.clone()).await?;
        }
        Ok(ctx)
    }
}
