use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::prelude::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct ToLowerCaseItem {}

impl ToLowerCaseItem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Item for ToLowerCaseItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.get_value() {
            Value::String(s) =>
                Ok(ctx.with_value(Value::String(s.to_lowercase()))),
            _ => Err(ctx.internal_server_error("lowercase: value is not string"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn lowercase_works() {
        let ctx = PipelineCtx::initial_state_with_value(Value::String(String::from("AbcD")));
        assert_eq!(
            ToLowerCaseItem::new()
                .call(ctx.clone())
                .await
                .unwrap()
                .value
                .as_str()
                .unwrap(),
            "abcd");
    }

    #[tokio::test]
    async fn should_check_ctx_value() {
        let ctx = PipelineCtx::initial_state_with_value(Value::Null);
        let r = ToLowerCaseItem::new().call(ctx.clone()).await;
        assert!(r.is_err());
    }
}
