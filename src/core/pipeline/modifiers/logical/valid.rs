
use async_trait::async_trait;

use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;


#[derive(Debug, Copy, Clone)]
pub struct ValidModifier { }

impl ValidModifier {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl Modifier for ValidModifier {

    fn name(&self) -> &'static str {
        "valid"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        ctx
    }
}
