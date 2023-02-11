use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;

use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct RegexReplaceModifier {
    format: Value,
    substitute: Value,
}

impl RegexReplaceModifier {
    pub fn new(format: impl Into<Value>, substitute: impl Into<Value>) -> Self {
        return RegexReplaceModifier {
            format: format.into(),
            substitute: substitute.into(),
        };
    }
}

#[async_trait]
impl Item for RegexReplaceModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let arg = self.format.resolve(ctx.clone()).await;
        let regex = arg.as_regexp().unwrap();
        let s_arg = self.substitute.resolve(ctx.clone()).await;
        let substitute = s_arg.as_str().unwrap();
        match &ctx.value {
            Value::String(s) => ctx.with_value(Value::String(regex.replace(s, substitute).to_string())),
            _ => ctx.invalid("Value is not string.")
        }
    }
}
