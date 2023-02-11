use async_trait::async_trait;

use crate::core::pipeline::item::Item;

use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct OrModifier {
    value: Value
}

impl OrModifier {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[async_trait]
impl Item for OrModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        if ctx.value.is_null() {
            ctx
        } else {
            match &self.value {
                Value::Pipeline(p) => p.process(ctx).await,
                _ => ctx.with_value(self.value.clone()),
            }
        }
    }
}
