use async_trait::async_trait;
use regex::Regex;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
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

    async fn call(&self, ctx: Context) -> Context {
        let format = self.format.resolve(ctx.clone()).await.as_str().unwrap();
        let substitute = self.substitute.resolve(ctx.clone()).await.as_str().unwrap();
        let regex = Regex::new(format).unwrap();
        match &ctx.value {
            Value::String(s) => ctx.alter_value(Value::String(regex.replace(s, substitute).to_string())),
            _ => ctx.invalid("Value is not string.")
        }
    }
}
