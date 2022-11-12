use async_trait::async_trait;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct EllipsisModifier {
    ellipsis: String,
    width: FunctionArgument,
}

impl EllipsisModifier {
    pub fn new(ellipsis: impl Into<String>, width: impl Into<FunctionArgument>) -> Self {
        Self { ellipsis: ellipsis.into(), width: width.into() }
    }
}

#[async_trait]
impl Modifier for EllipsisModifier {

    fn name(&self) -> &'static str {
        "ellipsis"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.width.resolve(ctx.clone()).await;
                let width = arg.as_u32().unwrap() as usize;
                if s.len() <= width {
                    ctx
                } else {
                    let ellipsis = &self.ellipsis;
                    ctx.alter_value(Value::String(s.chars().take(width).collect::<String>() + ellipsis))
                }
            }
        }
    }
}
