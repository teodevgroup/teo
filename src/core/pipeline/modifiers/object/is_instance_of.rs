use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::context::Context;

#[derive(Debug, Clone)]
pub struct IsObjectOfModifier {
    model: String
}

impl IsObjectOfModifier {
    pub fn new(model: impl Into<String>) -> Self {
        IsObjectOfModifier { model: model.into() }
    }
}

#[async_trait]
impl Modifier for IsObjectOfModifier {

    fn name(&self) -> &'static str {
        "isObjectOf"
    }

    async fn call<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        match ctx.value.as_hashmap() {
            Some(o) => {
                if o.model().name() == self.model {
                    ctx
                } else {
                    let model = &self.model;
                    ctx.invalid(format!("Value is not object of '{model}'."))
                }
            }
            None => ctx.invalid("Value is not object.")
        }
    }
}
