use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;


#[derive(Debug, Clone)]
pub struct ItemAtModifier {
    index: Value
}

impl ItemAtModifier {
    pub fn new(index: impl Into<Value>) -> Self {
        Self { index: index.into() }
    }
}

#[async_trait]
impl Item for ItemAtModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value.as_vec() {
            None => ctx.invalid("Value is not vector."),
            Some(v) => {
                let arg = self.index.resolve(ctx.clone()).await;
                let index = arg.as_i32().unwrap() as usize;
                let new_keypath = ctx.path.as_ref() + index as usize;
                ctx.with_value(v.get(index).unwrap().clone())
                   .with_path(new_keypath)
            }
        }
    }
}
