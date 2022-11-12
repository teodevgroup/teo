use async_trait::async_trait;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct HasSuffixModifier {
    suffix: FunctionArgument
}

impl HasSuffixModifier {
    pub fn new(suffix: impl Into<FunctionArgument>) -> Self {
        Self { suffix: suffix.into() }
    }
}

#[async_trait]
impl Modifier for HasSuffixModifier {

    fn name(&self) -> &'static str {
        "hasSuffix"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
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
