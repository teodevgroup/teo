use async_trait::async_trait;
use crate::core::item::Item;
use teo_teon::value::Value;

use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct AppendItem {
    argument: Value
}

impl AppendItem {
    pub fn new(argument: impl Into<Value>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for AppendItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        match &ctx.get_value() {
            Value::String(s) => {
                match argument.as_str() {
                    Some(a) => Ok(ctx.with_value(Value::String(s.to_owned() + a))),
                    None => Err(ctx.internal_server_error("Argument does not resolve to string."))
                }
            }
            Value::Vec(v) => {
                let mut v = v.clone();
                v.push(argument);
                Ok(ctx.with_value(Value::Vec(v)))
            }
            _ => Err(ctx.internal_server_error("Value is not string or vector."))
        }
    }
}
