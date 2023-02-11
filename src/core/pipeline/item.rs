use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;

#[async_trait]
pub trait Item: Debug + Send + Sync {

    // fn new(args: Vec<Argument>, table: Arc<Mutex<CallbackLookupTable>>) -> Self where Self: Sized;

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>>;
}
