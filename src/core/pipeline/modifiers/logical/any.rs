use async_trait::async_trait;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;


#[derive(Debug, Clone)]
pub struct AnyModifier {
    pipelines: Vec<Pipeline>
}

impl AnyModifier {
    pub fn new<F: Fn(&mut PipelineBuilder)>(build: F) -> Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        let pipelines: Vec<Pipeline> = pipeline.modifiers.iter().map(|modifier| {
            let mut p = PipelineBuilder::new();
            p.modifiers.push(modifier.clone());
            p.build()
        }).collect();
        return AnyModifier { pipelines };
    }
}


#[async_trait]
impl Modifier for AnyModifier {

    fn name(&self) -> &'static str {
        "any"
    }

    async fn call(&self, ctx: Context) -> Context {
        for pipeline in &self.pipelines {
            let result = pipeline.process(ctx.clone()).await;
            if result.is_valid() {
                return result;
            }
        }
        ctx.invalid("None of validators are valid.")
    }
}
