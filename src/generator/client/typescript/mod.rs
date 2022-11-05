pub mod pkg;
pub mod r#type;

use async_trait::async_trait;
use crate::generator::client::typescript::pkg::src::filter_ts::generate_filter_ts;
use crate::generator::client::typescript::pkg::src::index_ts::generate_index_ts;
use crate::generator::client::typescript::pkg::src::operation_ts::generate_operation_ts;
use crate::generator::client::typescript::pkg::src::runtime_ts::generate_runtime_ts;
use crate::core::conf::client::Client;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;

pub(crate) struct TypeScriptClientGenerator { }

#[async_trait]
impl ClientGenerator for TypeScriptClientGenerator {
    async fn generate_main(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("client/typescript/src/index.ts", generate_index_ts(graph, client.as_typescript()).await).await
    }

    async fn generate_accessories(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_directory("client").await?;
        generator.clear_directory("client/typescript").await?;
        generator.ensure_directory("client/typescript/src").await?;
        generator.generate_file("client/typescript/src/filter.ts", generate_filter_ts(graph).await).await?;
        generator.generate_file("client/typescript/src/operation.ts", generate_operation_ts(graph).await).await?;
        generator.generate_file("client/typescript/src/runtime.ts", generate_runtime_ts(graph, client.as_typescript()).await).await
    }
}
