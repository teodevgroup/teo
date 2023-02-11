use async_trait::async_trait;
use crate::core::teon::Value;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
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
impl Item for ObjectSetValueModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let key = self.key.resolve(ctx.clone()).await?;
        let value = self.value.resolve(ctx.clone()).await?;
        ctx.object.as_ref().unwrap().set_value(key.as_str().unwrap(), value).unwrap();
        ctx
    }
}
