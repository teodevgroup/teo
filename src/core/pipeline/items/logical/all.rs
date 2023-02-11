use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct AllModifier {
    pipelines: Vec<Pipeline>
}

impl AllModifier {
    pub fn new(pipelines: Vec<Pipeline>) -> Self {
        return Self { pipelines };
    }
}

#[async_trait]
impl Item for AllModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        for pipeline in &self.pipelines {
            if let Some(reason) = pipeline.process(context.clone()).await.invalid_reason() {
                return context.invalid(reason);
            }
        }
        context
    }
}
