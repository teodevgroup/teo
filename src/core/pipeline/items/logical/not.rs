use async_trait::async_trait;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::ctx::validity::Validity;
use crate::prelude::Value;


#[derive(Debug, Clone)]
pub struct NotModifier {
    value: Value
}

impl NotModifier {
    pub fn new(value: Value) -> Self {
        return NotModifier {
            value
        };
    }
}

#[async_trait]
impl Item for NotModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match &self.value {
            Value::Bool(b) => if *b {
                ctx.invalid("Value is invalid")
            } else {
                ctx.with_validity(Validity::Valid)
            }
            Value::Pipeline(p) => if p.process(ctx.clone()).await.is_valid() {
                ctx.invalid("Value is invalid.")
            } else {
                ctx
            }
            _ => panic!("Argument to `not` should be null, bool or pipeline.")
        }
    }
}
