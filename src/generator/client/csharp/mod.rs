use async_trait::async_trait;
use crate::app::app::ClientConfiguration;
use crate::generator::client::csharp::pkg::enumerable::generate_enumerable_cs;
use crate::generator::client::csharp::pkg::filters::generate_filters_cs;
use crate::generator::client::csharp::pkg::index::generate_index_cs;
use crate::generator::client::csharp::pkg::json_serializer::generate_json_serializer_cs;
use crate::generator::client::csharp::pkg::one_of::generate_one_of_cs;
use crate::generator::client::csharp::pkg::operations::generate_operations_cs;
use crate::generator::client::csharp::pkg::optional::generate_optional_cs;
use crate::generator::client::csharp::pkg::runtime::generate_runtime_cs;
use crate::generator::client::csharp::pkg::sort_order::generate_sort_order_cs;
use crate::generator::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::core::conf::client::Client;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;

pub mod r#type;
pub mod pkg;

pub(crate) struct CSharpClientGenerator { }

#[async_trait]
impl ClientGenerator for CSharpClientGenerator {
    async fn generate_main(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("client/csharp/Index.cs", generate_index_cs(graph, conf).await).await
    }

    async fn generate_accessories(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_directory("client").await?;
        generator.clear_directory("client/csharp").await?;
        generator.ensure_directory("client/csharp").await?;
        generator.generate_file("client/csharp/OneOf.cs", generate_one_of_cs(graph).await).await?;
        generator.generate_file("client/csharp/Optional.cs", generate_optional_cs(graph).await).await?;
        generator.generate_file("client/csharp/Enumerable.cs", generate_enumerable_cs(graph).await).await?;
        generator.generate_file("client/csharp/JsonSerializers.cs", generate_json_serializer_cs(graph).await).await?;
        generator.generate_file("client/csharp/Filters.cs", generate_filters_cs(graph).await).await?;
        generator.generate_file("client/csharp/Operations.cs", generate_operations_cs(graph).await).await?;
        generator.generate_file("client/csharp/SortOrder.cs", generate_sort_order_cs(graph).await).await?;
        generator.generate_file("client/csharp/Runtime.cs", generate_runtime_cs(graph, conf).await).await
    }
}
