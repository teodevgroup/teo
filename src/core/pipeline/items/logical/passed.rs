use async_trait::async_trait;
use crate::core::error::ErrorType;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct PassedModifier {
    pipeline: Pipeline
}

impl PassedModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return PassedModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Item for PassedModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match self.pipeline.process(ctx.clone()).await {
            Ok(_) => Ok(ctx.with_value(Value::Bool(true))),
            Err(error) => if error.r#type == ErrorType::InternalServerError {
                Err(error)
            } else {
                Ok(ctx.with_value(Value::Bool(false)))
            }
        }
    }
}
