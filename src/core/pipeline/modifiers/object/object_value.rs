use async_trait::async_trait;
use crate::core::key_path::KeyPathItem;
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

    async fn call(&self, ctx: Context) -> Context {
        let key = self.key.resolve(ctx.clone()).await;
        let value = ctx.object.get_value(key.as_str().unwrap()).unwrap();
        ctx.alter_value(value).alter_key_path(vec![KeyPathItem::String(key.as_string().unwrap())])
    }
}
