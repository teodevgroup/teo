use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct IsPrefixOfModifier {
    full: Value
}

impl IsPrefixOfModifier {
    pub fn new(full: impl Into<Value>) -> Self {
        Self { full: full.into() }
    }
}

#[async_trait]
impl Modifier for IsPrefixOfModifier {

    fn name(&self) -> &'static str {
        "isPrefixOf"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.full.resolve(ctx.clone()).await;
                let full = arg.as_str().unwrap();
                if full.starts_with(s) {
                    ctx
                } else {
                    ctx.invalid(format!("Value is not prefix of '{full}'."))
                }
            }
        }
    }
}
