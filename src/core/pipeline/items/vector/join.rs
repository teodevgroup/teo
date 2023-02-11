use async_trait::async_trait;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct JoinModifier {
    separator: Value
}

impl JoinModifier {
    pub fn new(separator: impl Into<Value>) -> Self {
        Self { separator: separator.into() }
    }
}

#[async_trait]
impl Item for JoinModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        match ctx.value.as_vec() {
            None => ctx.invalid("Value is not vector."),
            Some(v) => {
                let arg = self.separator.resolve(ctx.clone()).await;
                let separator = arg.as_str().unwrap();
                ctx.with_value(Value::String(v.iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>().join(separator)))
            }
        }
    }
}
