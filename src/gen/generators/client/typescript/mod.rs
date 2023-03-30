pub mod pkg;
pub mod r#type;

use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::gen::generators::client::typescript::pkg::src::index_d_ts::generate_index_d_ts;
use crate::core::graph::Graph;
use crate::gen::generators::client::ClientGenerator;
use crate::gen::generators::client::typescript::pkg::gitignore::generate_gitignore_ts;
use crate::gen::generators::client::typescript::pkg::package_json::{generate_package_json, update_package_json};
use crate::gen::generators::client::typescript::pkg::readme::generate_readme_ts;
use crate::gen::generators::client::typescript::pkg::src::index_js::generate_index_js;
use crate::gen::lib::generator::Generator;

pub(crate) struct TypeScriptClientGenerator { }

impl TypeScriptClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for TypeScriptClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        generator.generate_file("decimal.js", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/ts/src/decimal.js"))).await?;
        generator.generate_file("decimal.d.ts", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/ts/src/decimal.d.ts"))).await
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
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

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("index.d.ts", generate_index_d_ts(graph, client.object_name.clone(), false)).await?;
        generator.generate_file("index.js", generate_index_js(graph, client).await).await?;
        Ok(())
    }
}
