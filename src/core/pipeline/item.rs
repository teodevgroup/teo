use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::pipeline::ctx::Ctx;

#[async_trait]
pub trait Item: Debug + Send + Sync {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a>;
}
