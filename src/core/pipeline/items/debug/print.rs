use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct PrintModifier { }

impl PrintModifier {
    pub fn new() -> Self {
        return PrintModifier {};
    }
}

#[async_trait]
impl Item for PrintModifier {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        println!("{:?}", ctx.value);
        Ok(ctx)
    }
}
