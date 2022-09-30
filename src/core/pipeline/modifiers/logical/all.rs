use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct AllModifier {
    pipelines: Vec<Pipeline>
}

impl AllModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        let pipelines: Vec<Pipeline> = pipeline.modifiers.iter().map(|modifier| {
            let mut p = PipelineBuilder::new();
            p.modifiers.push(modifier.clone());
            p.build()
        }).collect();
        return AllModifier { pipelines };
    }
}

#[async_trait]
impl Modifier for AllModifier {

    fn name(&self) -> &'static str {
        "all"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        for pipeline in &self.pipelines {
            if let Some(reason) = pipeline.process(context.clone()).await.invalid_reason() {
                return context.invalid(reason);
            }
        }
        context
    }
}
