pub(crate) mod pkg;
pub(crate) mod teo;
pub(crate) mod types;

use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::generator::client::csharp::pkg::enumerable::generate_enumerable_cs;
use crate::generator::client::csharp::pkg::filters::generate_filters_cs;
use crate::generator::client::csharp::pkg::index::generate_index_cs;
use crate::generator::client::csharp::pkg::json_serializer::generate_json_serializer_cs;
use crate::generator::client::csharp::pkg::one_of::generate_one_of_cs;
use crate::generator::client::csharp::pkg::operations::generate_operations_cs;
use crate::generator::client::csharp::pkg::optional::generate_optional_cs;
use crate::generator::client::csharp::pkg::runtime::generate_runtime_cs;
use crate::generator::client::csharp::pkg::sort_order::generate_sort_order_cs;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;

pub(crate) struct CSharpClientGenerator { }

impl CSharpClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for CSharpClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("OneOf.cs", generate_one_of_cs(graph).await).await?;
        generator.generate_file("Optional.cs", generate_optional_cs(graph).await).await?;
        generator.generate_file("Enumerable.cs", generate_enumerable_cs(graph).await).await?;
        generator.generate_file("JsonSerializers.cs", generate_json_serializer_cs(graph).await).await?;
        generator.generate_file("Filters.cs", generate_filters_cs(graph).await).await?;
        generator.generate_file("Operations.cs", generate_operations_cs(graph).await).await?;
        generator.generate_file("SortOrder.cs", generate_sort_order_cs(graph).await).await?;
        generator.generate_file("Runtime.cs", generate_runtime_cs(graph, client).await).await
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("Index.cs", generate_index_cs(graph, client).await).await
    }
}
