pub mod pkg;
pub mod r#type;

use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::generator::client::typescript::pkg::src::filter_d_ts::generate_filter_d_ts;
use crate::generator::client::typescript::pkg::src::index_d_ts::generate_index_d_ts;
use crate::generator::client::typescript::pkg::src::operation_d_ts::generate_operation_d_ts;
use crate::generator::client::typescript::pkg::src::runtime_d_ts::generate_runtime_d_ts;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::client::typescript::pkg::src::index_js::generate_index_js;
use crate::generator::lib::generator::Generator;

pub(crate) struct TypeScriptClientGenerator { }

impl TypeScriptClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for TypeScriptClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("filter.d.ts", generate_filter_d_ts(graph).await).await?;
        generator.generate_file("operation.d.ts", generate_operation_d_ts(graph).await).await?;
        generator.generate_file("runtime.d.ts", generate_runtime_d_ts(graph, client).await).await
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        // gitignore
        // package.json
        // tslint.json
        Ok(())
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("index.d.ts", generate_index_d_ts(graph, client).await).await?;
        generator.generate_file("index.js", generate_index_js(graph, client).await).await?;
        Ok(())
    }
}
