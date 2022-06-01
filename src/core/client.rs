use std::fmt::Debug;
use async_trait::async_trait;
use crate::core::graph::Graph;


#[async_trait]
pub(crate) trait Client: Debug + Send + Sync {
    async fn generate(&self, graph: &'static Graph) -> std::io::Result<()>;
}
