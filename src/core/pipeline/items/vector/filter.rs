use async_trait::async_trait;
use crate::core::error::ErrorType;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct FilterModifier {
    pipeline: Pipeline
}

impl FilterModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        return FilterModifier {
            pipeline
        };
    }
}

#[async_trait]
impl Item for FilterModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let mut retval = Vec::new();
        for (i, val) in ctx.value.as_vec().unwrap().iter().enumerate() {
            let item_ctx = ctx.with_value(val.clone()).with_path(&ctx.path + i);
            let result = self.pipeline.process(item_ctx.clone()).await;
            match result {
                Ok(_) => retval.push(item_ctx.value.clone()),
                Err(error) => {
                    if error.r#type == ErrorType::InternalServerError {
                        return Err(error);
                    }
                }
            }
        }
        Ok(ctx.with_value(Value::Vec(retval)))
    }
}
