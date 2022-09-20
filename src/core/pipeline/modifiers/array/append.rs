use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct AppendModifier {
    argument: Argument
}

impl AppendModifier {
    pub fn new(argument: impl Into<Argument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for AppendModifier {

    fn name(&self) -> &'static str {
        "append"
    }

    async fn call(&self, ctx: Context) -> Context {
        let argument = self.argument.resolve(ctx.clone()).await;
        match &ctx.value {
            Value::String(s) => {
                match argument.as_str() {
                    Some(a) => ctx.alter_value(Value::String(s.to_owned() + a)),
                    None => ctx.invalid("Argument does not resolve to string.")
                }
            }
            Value::Vec(v) => {
                let mut v = v.clone();
                v.push(argument);
                ctx.alter_value(Value::Vec(v))
            }
            _ => ctx.invalid("Value is not string or vector.")
        }
    }
}
