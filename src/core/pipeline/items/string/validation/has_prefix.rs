use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

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
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.prefix.resolve(ctx.clone()).await;
                let prefix = arg.as_str().unwrap();
                if s.starts_with(prefix) {
                    ctx
                } else {
                    ctx.invalid(format!("Value is not prefixed by '{prefix}'."))
                }
            }
        }
    }
}
