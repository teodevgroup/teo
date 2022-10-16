use async_trait::async_trait;
use key_path::KeyPath;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;
use crate::core::tson::utils::TsonUtils;

#[derive(Debug, Clone)]
pub struct TsonSetModifier<'a> {
    path: KeyPath<'a>,
    argument: Argument
}

impl<'a> TsonSetModifier<'a> {
    pub fn new(path: KeyPath<'a>, argument: impl Into<Argument>) -> Self {
        Self { path, argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for TsonSetModifier<'_> {

    fn name(&self) -> &'static str {
        "tsonSet"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(ctx.clone()).await;
        match &ctx.value {
            Value::HashMap(_) => {
                ctx.alter_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            Value::BTreeMap(_) => {
                ctx.alter_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            Value::Vec(_) => {
                ctx.alter_value(TsonUtils::value_set(&ctx.value, self.path.clone(), &argument).unwrap())
            }
            _ => ctx.invalid("Value is not collection.")
        }
    }
}
