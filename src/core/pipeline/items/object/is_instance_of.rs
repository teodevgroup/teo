use async_trait::async_trait;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::ctx::Ctx;

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
impl Item for IsObjectOfModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value.as_object() {
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
