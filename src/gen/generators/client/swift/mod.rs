pub(crate) mod types;
pub(crate) mod teo;

use async_trait::async_trait;
use crate::core::graph::Graph;
use crate::gen::generators::client::swift::teo::generate_teo_swift;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::file_util::FileUtil;

pub(crate) struct SwiftClientGenerator { }

impl SwiftClientGenerator {
    pub(crate) fn new() -> Self { Self { } }
}

#[async_trait]
impl Generator for SwiftClientGenerator {
    fn module_directory_in_package(&self, conf: &Conf) -> String {
        return format!("Sources/{}", conf.package_name.as_ref().unwrap())
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("README.md", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/readme.md"))).await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/gitignore"))).await?;
        generator.generate_file("Package.swift", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/package.swift"))).await?;
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.generate_file("Teo.swift", generate_teo_swift(ctx)).await?;
        Ok(())
    }
}
