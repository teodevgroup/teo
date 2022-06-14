use std::fmt::Debug;
use async_trait::async_trait;


#[async_trait]
pub(crate) trait SaveSession: Debug + Send + Sync { }
