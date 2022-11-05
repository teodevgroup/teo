use crate::app::app::ClientConfiguration;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::generator::client::swift::pkg::gitignore::generate_gitignore;
use crate::generator::client::swift::pkg::package_swift::generate_package_swift;
use crate::generator::client::swift::pkg::readme_md::generate_readme_md;
use crate::core::conf::client::Client;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;

pub(crate) mod pkg;

pub(crate) struct SwiftClientGenerator { }

#[async_trait]
impl ClientGenerator for SwiftClientGenerator {
    async fn generate_main(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }

    async fn generate_accessories(graph: &Graph, client: &Client, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_directory("client").await?;
        generator.clear_directory("client/swift").await?;
        generator.generate_file("client/swift/README.md", generate_readme_md(graph).await).await?;
        generator.generate_file("client/swift/.gitignore", generate_gitignore(graph).await).await?;
        generator.generate_file("client/swift/Package.swift", generate_package_swift(graph).await).await
    }
}
