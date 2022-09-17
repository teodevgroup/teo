use crate::app::app::ClientConfiguration;
use crate::client::csharp::pkg::enumerable::generate_enumerable_cs;
use crate::client::csharp::pkg::filters::generate_filters_cs;
use crate::client::csharp::pkg::index::generate_index_cs;
use crate::client::csharp::pkg::json_serializer::generate_json_serializer_cs;
use crate::client::csharp::pkg::one_of::generate_one_of_cs;
use crate::client::csharp::pkg::operations::generate_operations_cs;
use crate::client::csharp::pkg::optional::generate_optional_cs;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::core::graph::Graph;

pub mod r#type;
pub mod pkg;

pub async fn generate_csharp_client(graph: &Graph, conf: &ClientConfiguration) -> std::io::Result<()> {
    ensure_directory("client").await?;
    clear_directory("client/csharp").await?;
    ensure_directory("client/csharp").await?;
    generate_file("client/csharp/OneOf.cs", generate_one_of_cs(graph).await).await?;
    generate_file("client/csharp/Optional.cs", generate_optional_cs(graph).await).await?;
    generate_file("client/csharp/Enumerable.cs", generate_enumerable_cs(graph).await).await?;
    generate_file("client/csharp/JsonSerializers.cs", generate_json_serializer_cs(graph).await).await?;
    generate_file("client/csharp/Filters.cs", generate_filters_cs(graph).await).await?;
    generate_file("client/csharp/Operations.cs", generate_operations_cs(graph).await).await?;
    generate_file("client/csharp/Index.cs", generate_index_cs(graph, conf).await).await
}
