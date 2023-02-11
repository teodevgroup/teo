use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

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
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let mut retval = Vec::new();
        for (i, val) in ctx.value.as_vec().unwrap().iter().enumerate() {
            let item_ctx = ctx.with_value(val.clone()).with_path(&ctx.path + i);
            if self.pipeline.process(item_ctx.clone()).await.is_valid() {
                retval.push(item_ctx.value.clone());
            }
        }
        ctx.with_value(Value::Vec(retval))
    }
}
