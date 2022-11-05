pub mod kotlin;
pub mod swift;
pub mod typescript;
pub mod dart;
pub mod csharp;

use async_trait::async_trait;
use crate::core::conf::client::Client;
use crate::generator::lib::generator::Generator;
use crate::core::graph::Graph;

#[async_trait]
pub(crate) trait ClientGenerator {
    async fn generate_main(&self, graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()>;
    async fn generate_accessories(&self, graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()>;
}
