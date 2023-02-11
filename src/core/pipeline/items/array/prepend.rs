use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct PrependModifier {
    argument: Value
}

impl PrependModifier {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for PrependModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match &ctx.value {
            Value::String(s) => {
                match argument.as_str() {
                    Some(a) => ctx.with_value(Value::String(a.to_string() + s)),
                    None => ctx.internal_server_error("Argument does not resolve to string.")
                }
            }
            Value::Vec(v) => {
                let mut v = v.clone();
                v.insert(0, argument);
                ctx.with_value(Value::Vec(v))
            }
            _ => ctx.internal_server_error("Value is not string or vector.")
        }
    }
}
