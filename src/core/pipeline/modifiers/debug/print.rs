use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct PrintModifier { }

impl PrintModifier {
    pub fn new() -> Self {
        return PrintModifier {};
    }
}

#[async_trait]
impl Modifier for PrintModifier {

    fn name(&self) -> &'static str {
        "print"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        println!("{:?}", context.value);
        context
    }
}
