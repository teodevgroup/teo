use async_trait::async_trait;
use key_path::KeyPath;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::utils::TsonUtils;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct TsonSetDefaultModifier<'a> {
    path: KeyPath<'a>,
    argument: Value
}

impl<'a> TsonSetDefaultModifier<'a> {
    pub fn new(path: KeyPath<'a>, argument: impl Into<Value>) -> Self {
        Self { path, argument: argument.into() }
    }
}

#[async_trait]
impl Item for TsonSetDefaultModifier<'_> {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match &ctx.value {
            Value::HashMap(_) => {
                ctx.with_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            Value::BTreeMap(_) => {
                ctx.with_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            Value::Vec(_) => {
                ctx.with_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            _ => ctx.internal_server_error("Value is not collection.")
        }
    }
}
