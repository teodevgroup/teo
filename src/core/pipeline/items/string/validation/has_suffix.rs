use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;

#[derive(Debug, Clone)]
pub struct HasSuffixModifier {
    suffix: Value
}

impl HasSuffixModifier {
    pub fn new(suffix: impl Into<Value>) -> Self {
        Self { suffix: suffix.into() }
    }
}

#[async_trait]
impl Item for HasSuffixModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value.as_str() {
            None => ctx.invalid("Value is not string."),
            Some(s) => {
                let arg = self.suffix.resolve(ctx.clone()).await;
                let suffix = arg.as_str().unwrap();
                if s.ends_with(suffix) {
                    ctx
                } else {
                    ctx.invalid(format!("Value is not suffixed by '{suffix}'."))
                }
            }
        }
    }
}
