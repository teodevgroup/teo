use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct RegexMatchModifier {
    argument: FunctionArgument
}

impl RegexMatchModifier {
    pub fn new(format: impl Into<FunctionArgument>) -> Self {
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

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
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
