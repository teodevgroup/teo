use async_trait::async_trait;
use crate::core::key_path::KeyPathItem;
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct ItemAtModifier {
    index: Argument
}

impl ItemAtModifier {
    pub fn new(index: impl Into<Argument>) -> Self {
        Self { index: index.into() }
    }
}

#[async_trait]
impl Modifier for ItemAtModifier {

    fn name(&self) -> &'static str {
        "itemAt"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value.as_vec() {
            None => ctx.invalid("Value is not vector."),
            Some(v) => {
                let arg = self.index.resolve(ctx.clone()).await;
                let index = arg.as_u32().unwrap() as usize;
                let mut new_keypath = ctx.key_path.clone();
                new_keypath.push(KeyPathItem::Number(index as usize));
                ctx.alter_value(v.get(index).unwrap().clone())
                   .alter_key_path(new_keypath)
            }
        }
    }
}
