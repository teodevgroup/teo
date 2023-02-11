use async_trait::async_trait;

use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct RegexMatchModifier {
    argument: Value
}

impl RegexMatchModifier {
    pub fn new(format: impl Into<Value>) -> Self {
        Self {
            argument: format.into()
        }
    }
}

#[async_trait]
impl Item for RegexMatchModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let arg_value = self.argument.resolve(ctx.clone()).await;
        let regex = arg_value.as_regexp().unwrap();
        match &ctx.value {
            Value::String(s) => {
                if regex.is_match(s) {
                    ctx.clone()
                } else {
                    ctx.invalid(format!("Value does not match '{regex}'"))
                }
            }
            _ => {
                ctx.invalid("Value is not string.")
            }
        }
    }
}
