use async_trait::async_trait;
use crate::core::key_path::KeyPathItem;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct ObjectPreviousValueModifier {
    key: Argument
}

impl ObjectPreviousValueModifier {
    pub fn new(key: impl Into<Argument>) -> Self {
        Self { key: key.into() }
    }
}

#[async_trait]
impl Modifier for ObjectPreviousValueModifier {

    fn name(&self) -> &'static str {
        "objectPreviousValue"
    }

    async fn call(&self, ctx: Context) -> Context {
        println!("here runs");
        let key = self.key.resolve(ctx.clone()).await;
        let value = ctx.object.get_previous_value(key.as_str().unwrap()).await.unwrap();
        ctx.alter_value(value).alter_key_path(vec![KeyPathItem::String(key.as_string().unwrap())])
    }
}
