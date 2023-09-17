use askama::Template;
use async_trait::async_trait;
use crate::core::result::Result;
use crate::gen::interface::client::conf::ClientConf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::filters;


#[derive(Template)]
#[template(path = "client/swift/readme.md.jinja", escape = "none")]
pub(self) struct SwiftReadMeTemplate<'a> {
    pub(self) conf: &'a ClientConf,
}

#[derive(Template)]
#[template(path = "client/swift/package.swift.jinja", escape = "none")]
pub(self) struct SwiftPackageSwiftTemplate<'a> {
    pub(self) conf: &'a ClientConf,
}

#[derive(Template)]
#[template(path = "client/swift/teo.swift.jinja", escape = "none")]
pub(self) struct SwiftMainTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a ClientConf,
}

pub(crate) struct SwiftClientGenerator { }

impl SwiftClientGenerator {
    pub(crate) fn new() -> Self { Self { } }
}

#[async_trait]
impl Generator for SwiftClientGenerator {
    fn module_directory_in_package(&self, conf: &ClientConf) -> String {
        return format!("Sources/{}", conf.inferred_package_name())
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.ensure_root_directory().await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/swift/gitignore"))).await?;
        generator.generate_file("README.md", SwiftReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
        generator.generate_file("Package.swift", SwiftPackageSwiftTemplate { conf: ctx.conf }.render().unwrap()).await?;
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.generate_file("Teo.swift", SwiftMainTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
        }.render().unwrap()).await
    }
}
