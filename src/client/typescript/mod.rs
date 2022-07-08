pub mod pkg;
pub mod r#type;

use async_trait::async_trait;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::app::app::ClientConfiguration;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::client::typescript::pkg::src::filter_ts::generate_filter_ts;
use crate::client::typescript::pkg::src::index_ts::generate_index_ts;
use crate::client::typescript::pkg::src::operation_ts::generate_operation_ts;
use crate::client::typescript::pkg::src::runtime_ts::generate_runtime_ts;
use crate::core::client::Client;
use crate::core::graph::Graph;


pub async fn generate_typescript_client(graph: &Graph, conf: &ClientConfiguration) -> std::io::Result<()> {
    ensure_directory("client").await?;
    clear_directory("client/typescript").await?;
    ensure_directory("client/typescript/src").await?;
    generate_file("client/typescript/src/filter.ts", generate_filter_ts(graph).await).await?;
    generate_file("client/typescript/src/operation.ts", generate_operation_ts(graph).await).await?;
    generate_file("client/typescript/src/runtime.ts", generate_runtime_ts(graph, conf).await).await?;
    generate_file("client/typescript/src/index.ts", generate_index_ts(graph).await).await
}