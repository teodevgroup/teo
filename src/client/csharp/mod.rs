pub mod r#type;
pub mod pkg;
use crate::app::app::ClientConfiguration;
use crate::client::csharp::pkg::filters::generate_filters_cs;
use crate::client::csharp::pkg::one_of::generate_one_of_cs;
use crate::client::csharp::pkg::optional::generate_optional_cs;
use crate::client::csharp::pkg::shared::generate_shared_cs;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::core::graph::Graph;

pub async fn generate_csharp_client(graph: &Graph, conf: &ClientConfiguration) -> std::io::Result<()> {
    ensure_directory("client").await?;
    clear_directory("client/csharp").await?;
    ensure_directory("client/csharp").await?;
    generate_file("client/csharp/src/OneOf.cs", generate_one_of_cs(graph).await).await?;
    generate_file("client/csharp/src/Optional.cs", generate_optional_cs(graph).await).await?;
    generate_file("client/csharp/src/Filters.cs", generate_filters_cs(graph).await).await
}
