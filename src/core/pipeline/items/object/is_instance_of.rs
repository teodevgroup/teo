use async_trait::async_trait;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
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
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_object() {
            Some(o) => {
                if o.model().name() == self.model {
                    Ok(ctx)
                } else {
                    let model = &self.model;
                    Err(ctx.internal_server_error(format!("value is not object of '{model}'.")))
                }
            }
            None => Err(ctx.internal_server_error("isObjectOf: value is not object"))
        }
    }
}
