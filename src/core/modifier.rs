use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::pipeline::stage::Stage;
use crate::core::object::Object;

#[async_trait]
pub trait Modifier: Debug + Send + Sync {
    fn name(&self) -> &'static str;
    async fn call(&self, stage: Stage, object: &Object) -> Stage;
}
