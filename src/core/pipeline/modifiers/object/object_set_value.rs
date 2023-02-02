use async_trait::async_trait;
use crate::core::teon::Value;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ObjectSetValueModifier {
    key: Value,
    value: Value,
}

impl ObjectSetValueModifier {
    pub fn new(key: impl Into<Value>, value: impl Into<Value>) -> Self {
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

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let key = self.key.resolve(ctx.clone()).await;
        let value = self.value.resolve(ctx.clone()).await;
        ctx.object.as_ref().unwrap().set_value(key.as_str().unwrap(), value).unwrap();
        ctx
    }
}
