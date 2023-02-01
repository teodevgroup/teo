use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::validity::Validity;
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
impl Modifier for NotModifier {

    fn name(&self) -> &'static str {
        "not"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match &self.value {
            Value::Bool(b) => if *b {
                ctx.invalid("Value is invalid")
            } else {
                ctx.alter_validity(Validity::Valid)
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
