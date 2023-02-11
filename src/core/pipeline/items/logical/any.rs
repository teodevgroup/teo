use async_trait::async_trait;
use crate::core::result::Result;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;


#[derive(Debug, Clone)]
pub struct AnyItem {
    pipelines: Vec<Pipeline>
}

impl AnyItem {
    pub fn new(pipelines: Vec<Pipeline>) -> Self {
        return Self { pipelines };
    }
}


#[async_trait]
impl Item for AnyItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        for pipeline in &self.pipelines {
            let result = pipeline.process(ctx.clone()).await;
            if result.is_ok() {
                return Ok(ctx.clone())
            }
        }
        Err(ctx.with_invalid("any of validators are invalid"))
    }
}
