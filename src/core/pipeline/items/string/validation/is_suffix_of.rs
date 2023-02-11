use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
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
impl Item for IsSuffixOfModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => ctx.internal_server_error("Value is not string."),
            Some(s) => {
                let arg = self.full.resolve(ctx.clone()).await?;
                let full = arg.as_str().unwrap();
                if full.ends_with(s) {
                    ctx
                } else {
                    ctx.internal_server_error(format!("Value is not suffix of '{full}'."))
                }
            }
        }
    }
}
