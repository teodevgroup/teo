use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::generator::client::swift::pkg::gitignore::generate_gitignore;
use crate::generator::client::swift::pkg::package_swift::generate_package_swift;
use crate::generator::client::swift::pkg::readme_md::generate_readme_md;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;
use crate::parser::ast::client::Client;

pub(crate) mod pkg;

pub(crate) struct SwiftClientGenerator { }

impl SwiftClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for SwiftClientGenerator {
    fn module_directory_in_package(&self, client: &ClientGeneratorConf) -> String {
        return "src".to_owned()
    }

    async fn generate_module_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("client/swift/README.md", generate_readme_md(graph).await).await?;
        generator.generate_file("client/swift/.gitignore", generate_gitignore(graph).await).await?;
        generator.generate_file("client/swift/Package.swift", generate_package_swift(graph).await).await?;
        Ok(())
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }
}
