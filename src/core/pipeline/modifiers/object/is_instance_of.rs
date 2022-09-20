use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct IsObjectOfModifier {
    model: &'static str
}

impl IsObjectOfModifier {
    pub fn new(model: &'static str) -> Self {
        IsObjectOfModifier { model }
    }
}

#[async_trait]
impl Modifier for IsObjectOfModifier {

    fn name(&self) -> &'static str {
        "isObjectOf"
    }

    async fn call(&self, ctx: Context) -> Context {
        match ctx.value.as_object() {
            Some(o) => {
                if o.model().name() == self.model {
                    ctx
                } else {
                    let model = self.model;
                    ctx.invalid(format!("Value is not object of '{model}'."))
                }
            }
            None => ctx.invalid("Value is not object.")
        }
    }
}
