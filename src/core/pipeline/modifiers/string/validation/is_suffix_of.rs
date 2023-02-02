use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::teon::Value;

#[derive(Debug, Clone)]
pub struct IsSuffixOfModifier {
    full: Value
}

impl IsSuffixOfModifier {
    pub fn new(full: impl Into<Value>) -> Self {
        Self { full: full.into() }
    }
}

#[async_trait]
impl Modifier for IsSuffixOfModifier {

    fn name(&self) -> &'static str {
        "isSuffixOf"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.full.resolve(ctx.clone()).await;
                let full = arg.as_str().unwrap();
                if full.ends_with(s) {
                    ctx
                } else {
                    ctx.invalid(format!("Value is not suffix of '{full}'."))
                }
            }
        }
    }
}
