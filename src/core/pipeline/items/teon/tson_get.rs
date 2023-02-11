use async_trait::async_trait;
use key_path::KeyPath;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::utils::TsonUtils;
use crate::core::result::Result;
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
impl Item for TsonGetModifier<'_> {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match &ctx.value {
            Value::HashMap(_) => {
                ctx.with_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            Value::BTreeMap(_) => {
                ctx.with_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            Value::Vec(_) => {
                ctx.with_value(TsonUtils::value_get(&ctx.value, self.path.clone()).unwrap())
            }
            _ => ctx.internal_server_error("Value is not collection.")
        }
    }
}
