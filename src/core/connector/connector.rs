use std::sync::Arc;
use async_trait::async_trait;
use crate::core::connector::connection::Connection;
use crate::core::result::Result;

#[async_trait]
pub(crate) trait Connector: Send + Sync {

    async fn connection(&self) -> Result<Arc<dyn Connection>>;

}
