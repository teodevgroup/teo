use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;

use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct PrependModifier {
    argument: FunctionArgument
}

impl PrependModifier {
    pub fn new(argument: impl Into<FunctionArgument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for PrependModifier {

    fn name(&self) -> &'static str {
        "prepend"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match &ctx.value {
            Value::String(s) => {
                match argument.as_str() {
                    Some(a) => ctx.alter_value(Value::String(a.to_string() + s)),
                    None => ctx.invalid("Argument does not resolve to string.")
                }
            }
            Value::Vec(v) => {
                let mut v = v.clone();
                v.insert(0, argument);
                ctx.alter_value(Value::Vec(v))
            }
            _ => ctx.invalid("Value is not string or vector.")
        }
    }
}
