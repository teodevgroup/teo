use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct RegexMatchModifier {
    argument: Argument
}

impl RegexMatchModifier {
    pub fn new(format: impl Into<Argument>) -> Self {
        Self {
            argument: format.into()
        }
    }
}

#[async_trait]
impl Modifier for RegexMatchModifier {

    fn name(&self) -> &'static str {
        "regexMatch"
    }

    async fn call(&self, ctx: Context) -> Context {
        let arg_value = self.argument.resolve(ctx.clone()).await;
        let format = arg_value.as_str().unwrap();
        let regex = Regex::new(format).unwrap();
        match &ctx.value {
            Value::String(s) => {
                if regex.is_match(s) {
                    ctx.clone()
                } else {
                    ctx.invalid(format!("Value does not match '{format}'"))
                }
            }
            _ => {
                ctx.invalid("Value is not string.")
            }
        }
    }
}
