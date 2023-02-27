use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct IsObjectOfItem {
    model: String
}

impl IsObjectOfItem {
    pub fn new(model: impl Into<String>) -> Self {
        IsObjectOfItem { model: model.into() }
    }
}

#[async_trait]
impl Item for IsObjectOfItem {
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
