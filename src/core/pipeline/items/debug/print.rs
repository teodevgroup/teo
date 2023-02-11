use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct PrintItem { }

impl PrintItem {
    pub fn new() -> Self {
        return PrintItem {};
    }
}

#[async_trait]
impl Item for PrintItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        println!("{:?}", ctx.value);
        Ok(ctx)
    }
}
