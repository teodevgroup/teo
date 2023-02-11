use async_trait::async_trait;
use key_path::path;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct ObjectPreviousValueModifier {
    key: Value
}

impl ObjectPreviousValueModifier {
    pub fn new(key: impl Into<Value>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl Item for ObjectPreviousValueModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let key = self.key.resolve(ctx.clone()).await?;
        let value = ctx.object.as_ref().unwrap().get_previous_value(key.str_from_string_or_raw_enum_choice().unwrap()).unwrap();
        ctx.with_value(value).with_path(path![key.as_str().unwrap().to_string()])
    }
}
