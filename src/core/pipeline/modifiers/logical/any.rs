use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;


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
impl Modifier for AnyModifier {

    fn name(&self) -> &'static str {
        "any"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        for pipeline in &self.pipelines {
            let result = pipeline.process(ctx.clone()).await;
            if result.is_valid() {
                return result;
            }
        }
        ctx.invalid("None of validators are valid.")
    }
}
