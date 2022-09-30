use async_trait::async_trait;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct IsSuffixOfModifier {
    full: Argument
}

impl IsSuffixOfModifier {
    pub fn new(full: impl Into<Argument>) -> Self {
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
