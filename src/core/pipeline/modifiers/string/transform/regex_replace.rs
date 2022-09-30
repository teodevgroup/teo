use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct RegexReplaceModifier {
    format: Argument,
    substitute: Argument,
}

impl RegexReplaceModifier {
    pub fn new(format: impl Into<Argument>, substitute: impl Into<Argument>) -> Self {
        return RegexReplaceModifier {
            format: format.into(),
            substitute: substitute.into(),
        };
    }
}

#[async_trait]
impl Modifier for RegexReplaceModifier {

    fn name(&self) -> &'static str {
        "regexReplace"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let arg = self.format.resolve(ctx.clone()).await;
        let format = arg.as_str().unwrap();
        let s_arg = self.substitute.resolve(ctx.clone()).await;
        let substitute = s_arg.as_str().unwrap();
        let regex = Regex::new(format).unwrap();
        match &ctx.value {
            Value::String(s) => ctx.alter_value(Value::String(regex.replace(s, substitute).to_string())),
            _ => ctx.invalid("Value is not string.")
        }
    }
}
