use crate::app::app::ClientConfiguration;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::client::swift::pkg::gitignore::generate_gitignore;
use crate::client::swift::pkg::package_swift::generate_package_swift;
use crate::client::swift::pkg::readme_md::generate_readme_md;
use crate::core::graph::Graph;

pub(crate) mod pkg;

pub async fn generate_swift_client(graph: &Graph, _conf: &ClientConfiguration) -> std::io::Result<()> {
    ensure_directory("client").await?;
    clear_directory("client/swift").await?;
    generate_file("client/swift/README.md", generate_readme_md(graph).await).await?;
    generate_file("client/swift/.gitignore", generate_gitignore(graph).await).await?;
    generate_file("client/swift/Package.swift", generate_package_swift(graph).await).await
}