use askama::Template;
use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::filters;

#[derive(Template)]
#[template(path = "client/dart/readme.md.jinja", escape = "none")]
pub(self) struct DartReadMeTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/dart/pubspec.yaml.jinja", escape = "none")]
pub(self) struct DartPubspecTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/dart/teo.dart.jinja", escape = "none")]
pub(self) struct DartMainTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a Conf,
}

pub(crate) struct DartClientGenerator { }

impl DartClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for DartClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        "lib".to_owned()
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/dart/gitignore"))).await?;
        generator.generate_file("README.md", DartReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
        generator.generate_file("pubspec.yaml", DartPubspecTemplate { conf: ctx.conf }.render().unwrap()).await?;
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.generate_file(format!("{}.dart", ctx.conf.inferred_package_name_snake_case()), DartMainTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
        }.render().unwrap()).await
    }
}
