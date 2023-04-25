use async_trait::async_trait;

#[async_trait]
pub trait Transaction: Send + Sync {

}