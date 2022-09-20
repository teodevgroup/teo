use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;
use crate::core::value::Value;

#[derive(Debug, Clone)]
pub struct ObjectValueModifier {
    key: Argument
}

impl ObjectValueModifier {
    pub fn new(key: impl Into<Argument>) -> Self {
        Self { key }
    }
}

#[async_trait]
impl Modifier for ObjectValueModifier {

    fn name(&self) -> &'static str {
        "objectValue"
    }

    async fn call(&self, ctx: Context) -> Context {
        let key = self.key.resolve(ctx).await;
        let value = ctx.object.get_value(key.as_str().unwrap()).unwrap();
        ctx.alter_value(value)
    }
}
