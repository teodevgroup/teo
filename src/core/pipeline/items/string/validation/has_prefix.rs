use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct HasPrefixModifier {
    prefix: Value
}

impl HasPrefixModifier {
    pub fn new(prefix: impl Into<Value>) -> Self {
        Self { prefix: prefix.into() }
    }
}

#[async_trait]
impl Item for HasPrefixModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_str() {
            None => ctx.internal_server_error("Value is not string."),
            Some(s) => {
                let arg = self.prefix.resolve(ctx.clone()).await?;
                let prefix = arg.as_str().unwrap();
                if s.starts_with(prefix) {
                    ctx
                } else {
                    ctx.internal_server_error(format!("Value is not prefixed by '{prefix}'."))
                }
            }
        }
    }
}
