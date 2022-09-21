use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct JoinModifier {
    separator: Argument
}

impl JoinModifier {
    pub fn new(separator: impl Into<Argument>) -> Self {
        Self { separator: separator.into() }
    }
}

#[async_trait]
impl Modifier for JoinModifier {

    fn name(&self) -> &'static str {
        "join"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value.as_vec() {
            None => ctx.invalid("Value is not vector."),
            Some(v) => {
                let arg = self.separator.resolve(ctx.clone()).await;
                let separator = arg.as_str().unwrap();
                ctx.alter_value(Value::String(v.iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>().join(separator)))
            }
        }
    }
}
