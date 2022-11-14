use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::tson::Value;

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
impl Modifier for HasPrefixModifier {

    fn name(&self) -> &'static str {
        "hasPrefix"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
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
