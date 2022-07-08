use async_trait::async_trait;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::app::app::ClientConfiguration;
use crate::core::client::Client;
use crate::core::graph::Graph;

pub async fn generate_kotlin_client(_graph: &Graph, _conf: &ClientConfiguration) -> std::io::Result<()> {
    Ok(())
}