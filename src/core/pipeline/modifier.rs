use std::fmt::Debug;
use async_trait::async_trait;

use crate::core::pipeline::context::Context;

#[async_trait]
pub trait Modifier: Debug + Send + Sync {
    fn name(&self) -> &'static str;
    async fn call(&self, ctx: Context) -> Context;
}
