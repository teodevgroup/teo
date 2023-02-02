use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::context::Context;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct AndModifier {
    value: Value
}

impl AndModifier {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[async_trait]
impl Modifier for AndModifier {

    fn name(&self) -> &'static str {
        "and"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        if !ctx.value.is_null() {
            ctx
        } else {
            match &self.value {
                Value::Pipeline(p) => p.process(ctx).await,
                _ => ctx.alter_value(self.value.clone()),
            }
        }
    }
}
