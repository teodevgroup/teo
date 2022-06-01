pub mod pkg;
pub mod r#type;

use async_trait::async_trait;
use std::cell::RefCell;
use std::fmt::Error;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::client::typescript::pkg::src::filter_ts::generate_filter_ts;
use crate::client::typescript::pkg::src::index_ts::generate_index_ts;
use crate::client::typescript::pkg::src::runtime_ts::generate_runtime_ts;
use crate::core::client::Client;
use crate::core::graph::Graph;


#[derive(Debug)]
pub struct TypeScriptClient {
    pub(crate) at: Arc<Mutex<String>>,
    pub(crate) react_hooks: AtomicBool,
}

impl TypeScriptClient {
    pub(crate) fn new() -> Self {
        TypeScriptClient {
            at: Arc::new(Mutex::new("".to_string())),
            react_hooks: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl Client for TypeScriptClient {
    async fn generate(&self, graph: &'static Graph) -> std::io::Result<()> {
        ensure_directory("client").await?;
        clear_directory("client/typescript").await?;
        ensure_directory("client/typescript/src").await?;
        generate_file("client/typescript/src/filter.ts", generate_filter_ts(graph).await).await?;
        generate_file("client/typescript/src/runtime.ts", generate_runtime_ts(graph).await).await?;
        generate_file("client/typescript/src/index.ts", generate_index_ts(graph).await).await
    }
}
