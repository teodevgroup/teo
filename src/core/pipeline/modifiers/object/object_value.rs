use async_trait::async_trait;
use key_path::path;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ObjectValueModifier {
    key: Argument
}

impl ObjectValueModifier {
    pub fn new(key: impl Into<Argument>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl Modifier for ObjectValueModifier {

    fn name(&self) -> &'static str {
        "objectValue"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let key = self.key.resolve(ctx.clone()).await;
        let value = ctx.object.get_value(key.as_str().unwrap()).unwrap();
        ctx.alter_value(value).alter_key_path(path![key.as_str().unwrap()])
    }
}
