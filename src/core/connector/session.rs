use std::fmt::Debug;
use async_trait::async_trait;

#[async_trait]
pub trait SaveSession: Debug + Send + Sync { }
