use async_trait::async_trait;
use key_path::KeyPath;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;
use crate::core::pipeline::argument::FunctionArgument;
use crate::core::pipeline::context::Context;
use crate::core::tson::utils::TsonUtils;

#[derive(Debug, Clone)]
pub struct TsonSetDefaultModifier<'a> {
    path: KeyPath<'a>,
    argument: FunctionArgument
}

impl<'a> TsonSetDefaultModifier<'a> {
    pub fn new(path: KeyPath<'a>, argument: impl Into<FunctionArgument>) -> Self {
        Self { path, argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for TsonSetDefaultModifier<'_> {

    fn name(&self) -> &'static str {
        "tsonSetDefault"
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
