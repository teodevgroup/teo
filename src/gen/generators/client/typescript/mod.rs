pub mod pkg;
pub mod r#type;
use crate::core::result::Result;
use async_trait::async_trait;
use crate::gen::generators::client::typescript::pkg::src::index_d_ts::generate_index_d_ts;
use crate::gen::generators::client::typescript::pkg::gitignore::generate_gitignore_ts;
use crate::gen::generators::client::typescript::pkg::package_json::{generate_package_json, update_package_json};
use crate::gen::generators::client::typescript::pkg::readme::generate_readme_ts;
use crate::gen::generators::client::typescript::pkg::src::index_js::generate_index_js;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::file_util::FileUtil;

pub(crate) struct TypeScriptClientGenerator { }

impl TypeScriptClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for TypeScriptClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.clear_root_directory().await
    }

    async fn generate_package_files(&self, _ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.ensure_root_directory().await?;
        generator.generate_file_if_not_exist(".gitignore", generate_gitignore_ts()).await?;
        generator.generate_file_if_not_exist("README.md", generate_readme_ts(generator.get_base_dir())).await?;
        if generator.generate_file_if_not_exist("package.json", generate_package_json(generator.get_base_dir())).await? {
            // if exist, update package.json with a minor version
            let json_data = std::fs::read_to_string(generator.get_file_path("package.json"))
                .expect("Unable to read package.json");
            generator.generate_file("package.json", update_package_json(json_data)).await?;
        }
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.generate_file("index.d.ts", generate_index_d_ts(ctx.graph, ctx.conf.object_name.clone(), false)).await?;
        generator.generate_file("index.js", generate_index_js(ctx.graph, ctx.conf).await).await?;
        Ok(())
    }
}
