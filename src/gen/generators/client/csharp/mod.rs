pub(crate) mod pkg;
pub(crate) mod teo;
pub(crate) mod types;

use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::gen::generators::client::csharp::pkg::runtime::generate_runtime_cs;
use crate::core::graph::Graph;
use crate::gen::generators::client::ClientGenerator;
use crate::gen::lib::file_util::FileUtil;

pub(crate) struct CSharpClientGenerator { }

impl CSharpClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for CSharpClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("Runtime.cs", generate_runtime_cs(graph, client).await).await
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &FileUtil) -> std::io::Result<()> {
        Ok(())
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &FileUtil) -> std::io::Result<()> {
        Ok(())
        //generator.generate_file("Index.cs", generate_index_cs(graph, client).await).await
    }
}
