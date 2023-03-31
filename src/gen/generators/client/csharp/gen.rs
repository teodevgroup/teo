use askama::Template;
use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::filters;

#[derive(Template)]
#[template(path = "client/csharp/readme.md.jinja", escape = "none")]
pub(self) struct CSharpReadMeTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/csharp/proj.sln.jinja", escape = "none")]
pub(self) struct CSharpSlnTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/csharp/teo.cs.jinja", escape = "none")]
pub(self) struct CSharpMainTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a Conf,
}

pub(crate) struct CSharpClientGenerator { }

impl CSharpClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for CSharpClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/csharp/gitignore"))).await?;
        generator.generate_file(format!("{}.csproj", ctx.conf.inferred_package_name()), include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/csharp/proj.csproj"))).await?;
        generator.generate_file("README.md", CSharpReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
        generator.generate_file(format!("{}.sln", ctx.conf.inferred_package_name()), CSharpSlnTemplate { conf: ctx.conf }.render().unwrap()).await?;
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.generate_file("Teo.cs", CSharpMainTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
        }.render().unwrap()).await
    }
}
