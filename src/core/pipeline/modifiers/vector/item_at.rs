use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;


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
impl Modifier for ItemAtModifier {

    fn name(&self) -> &'static str {
        "itemAt"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_vec() {
            None => ctx.invalid("Value is not vector."),
            Some(v) => {
                let arg = self.index.resolve(ctx.clone()).await;
                let index = arg.as_u32().unwrap() as usize;
                let new_keypath = ctx.key_path.as_ref() + index as usize;
                ctx.alter_value(v.get(index).unwrap().clone())
                   .alter_key_path(new_keypath)
            }
        }
    }
}
