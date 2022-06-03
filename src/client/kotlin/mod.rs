use async_trait::async_trait;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::core::client::Client;
use crate::core::graph::Graph;

#[derive(Debug)]
pub struct KotlinClient {
    pub(crate) at: Arc<Mutex<String>>,
    pub(crate) jetpack_compose_states: AtomicBool,
}

impl KotlinClient {
    pub(crate) fn new() -> Self {
        KotlinClient {
            at: Arc::new(Mutex::new("".to_string())),
            jetpack_compose_states: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl Client for KotlinClient {
    async fn generate(&self, _graph: &'static Graph) -> std::io::Result<()> {
        Ok(())
    }
}
