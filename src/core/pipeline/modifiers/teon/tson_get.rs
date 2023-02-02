use async_trait::async_trait;
use key_path::KeyPath;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;
use crate::core::pipeline::context::Context;
use crate::core::teon::utils::TsonUtils;

#[derive(Debug, Clone)]
pub struct TsonGetModifier<'a> {
    path: KeyPath<'a>,
}

impl<'a> TsonGetModifier<'a> {
    pub fn new(path: KeyPath<'a>) -> Self {
        Self { path }
    }
}

#[async_trait]
impl Modifier for TsonGetModifier<'_> {

    fn name(&self) -> &'static str {
        "tsonGet"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match &ctx.value {
            Value::HashMap(_) => {
                ctx.alter_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            Value::BTreeMap(_) => {
                ctx.alter_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            Value::Vec(_) => {
                ctx.alter_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            _ => ctx.invalid("Value is not collection.")
        }
    }
}
