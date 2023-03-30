pub(crate) mod types;
pub(crate) mod teo;

use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::graph::Graph;
use crate::gen::generators::client::ClientGenerator;
use crate::gen::generators::client::swift::teo::generate_teo_swift;
use crate::gen::internal::file_util::FileUtil;

pub(crate) struct SwiftClientGenerator { }

impl SwiftClientGenerator {
    pub(crate) fn new() -> Self { Self { } }
}

#[async_trait]
impl ClientGenerator for SwiftClientGenerator {
    fn module_directory_in_package(&self, client: &ClientGeneratorConf) -> String {
        return format!("Sources/{}", client.package_name.as_ref().unwrap())
    }

    async fn generate_module_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("README.md", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/readme.md"))).await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/gitignore"))).await?;
        generator.generate_file("Package.swift", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/package.swift"))).await?;
        Ok(())
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &FileUtil) -> std::io::Result<()> {
        generator.generate_file("Teo.swift", generate_teo_swift(graph, client)).await?;
        Ok(())
    }
}
