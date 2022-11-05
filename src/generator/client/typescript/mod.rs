pub mod pkg;
pub mod r#type;

use crate::app::app::ClientConfiguration;
use crate::generator::client::shared::{clear_directory, ensure_directory, generate_file};
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
        generate_file("client/typescript/src/index.ts", generate_index_ts(graph, conf).await).await
    }

    async fn generate_accessories(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        ensure_directory("client").await?;
        clear_directory("client/typescript").await?;
        ensure_directory("client/typescript/src").await?;
        generate_file("client/typescript/src/filter.ts", generate_filter_ts(graph).await).await?;
        generate_file("client/typescript/src/operation.ts", generate_operation_ts(graph).await).await?;
        generate_file("client/typescript/src/runtime.ts", generate_runtime_ts(graph, conf).await).await?
    }
}
