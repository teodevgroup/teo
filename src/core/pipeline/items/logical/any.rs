use async_trait::async_trait;

use crate::core::pipeline::item::Item;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;


#[derive(Debug, Clone)]
pub struct AnyModifier {
    pipelines: Vec<Pipeline>
}

impl AnyModifier {
    pub fn new(pipelines: Vec<Pipeline>) -> Self {
        return Self { pipelines };
    }
}


#[async_trait]
impl Item for AnyModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        for pipeline in &self.pipelines {
            let result = pipeline.process(ctx.clone()).await;
            if result.is_valid() {
                return result;
            }
        }
        ctx.invalid("None of validators are valid.")
    }
}
