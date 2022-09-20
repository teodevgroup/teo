use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsExistModifier {}

impl IsExistModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for IsExistModifier {

    fn name(&self) -> &'static str {
        "isExist"
    }

    async fn call(&self, ctx: Context) -> Context {
        println!("see here runs or not {:?} {:?}", ctx.key_path, ctx.value);
        if ctx.value.is_null() {
            ctx.invalid("Value is not exist.")
        } else {
            ctx
        }
    }
}
