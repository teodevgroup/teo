use std::env;
use std::process::Command;
use askama::Template;
use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::filters;
use crate::gen::internal::message::green_message;

#[derive(Template)]
#[template(path = "client/kotlin/readme.md.jinja", escape = "none")]
pub(self) struct KotlinReadMeTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/kotlin/build.gradle.kts.jinja", escape = "none")]
pub(self) struct KotlinBuildGradleTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/kotlin/settings.gradle.kts.jinja", escape = "none")]
pub(self) struct KotlinSettingsGradleTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/kotlin/teo.kt.jinja", escape = "none")]
pub(self) struct KotlinMainTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a Conf,
}

pub(crate) struct KotlinClientGenerator { }

impl KotlinClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for KotlinClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        "src/main/kotlin".to_owned()
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.clear_root_directory().await?;
        Ok(())
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        let base = generator.get_base_dir();
        let mut has_project = false;
        for file in ["build.gradle", "build.gradle.kts"] {
            let proj_file = base.join(file);
            if proj_file.exists() { has_project = true; }
        }
        if has_project {

        } else {
            let saved_cwd = env::current_dir().unwrap();
            env::set_current_dir(base).unwrap();
            green_message("run", "`gradle init --type basic`".to_owned());
            Command::new("gradle").arg("init").arg("--type").arg("basic").spawn()?.wait()?;
            env::set_current_dir(saved_cwd).unwrap();
            generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/kotlin/gitignore"))).await?;
            generator.generate_file("README.md", KotlinReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
            generator.generate_file("build.gradle.kts", KotlinBuildGradleTemplate { conf: ctx.conf }.render().unwrap()).await?;
            generator.generate_file("settings.gradle.kts", KotlinSettingsGradleTemplate { conf: ctx.conf }.render().unwrap()).await?;
        }
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.generate_file(format!("{}.kt", ctx.conf.inferred_package_name_camel_case()), KotlinMainTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
        }.render().unwrap()).await?;
        Ok(())
    }
}
