use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ObjectSetValueModifier {
    key: Argument,
    value: Argument,
}

impl ObjectSetValueModifier {
    pub fn new(key: impl Into<Argument>, value: impl Into<Argument>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[async_trait]
impl Modifier for ObjectSetValueModifier {

    fn name(&self) -> &'static str {
        "objectSetValue"
    }

    async fn call(&self, ctx: Context) -> Context {
        let key = self.key.resolve(ctx.clone()).await;
        let value = self.value.resolve(ctx.clone()).await;
        ctx.object.set_value(key.as_str().unwrap(), value).unwrap();
        ctx
    }
}
