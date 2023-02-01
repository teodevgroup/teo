use async_trait::async_trait;
use key_path::path;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::core::tson::Value;

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
impl Modifier for ObjectPreviousValueModifier {

    fn name(&self) -> &'static str {
        "objectPreviousValue"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let key = self.key.resolve(ctx.clone()).await;
        let value = ctx.object.as_ref().unwrap().get_previous_value(key.str_from_string_or_raw_enum_choice().unwrap()).unwrap();
        ctx.alter_value(value).alter_key_path(path![key.as_str().unwrap().to_string()])
    }
}
