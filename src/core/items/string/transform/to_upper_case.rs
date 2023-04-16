use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct ToUpperCaseItem {}

impl ToUpperCaseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ToUpperCaseItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.get_value() {
            Value::String(s) =>
                Ok(ctx.with_value(Value::String(s.to_uppercase()))),
            _ => Err(ctx.internal_server_error("uppercase: value is not string"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn uppercase_works() {
        let ctx = Ctx::initial_state_with_value(Value::String(String::from("AbcD")));
        assert_eq!(
            ToUpperCaseItem::new()
                .call(ctx.clone())
                .await
                .unwrap()
                .value
                .as_str()
                .unwrap(),
            "ABCD");
    }

    #[tokio::test]
    async fn should_check_ctx_value() {
        let ctx = Ctx::initial_state_with_value(Value::Null);
        let r = ToUpperCaseItem::new().call(ctx.clone()).await;
        assert!(r.is_err());
    }
}
