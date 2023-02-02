use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::generator::client::ClientGenerator;
use crate::generator::lib::generator::Generator;
use crate::parser::ast::client::Client;
use crate::prelude::Graph;

pub(crate) struct KotlinClientGenerator { }

impl KotlinClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for KotlinClientGenerator {
    fn module_directory_in_package(&self, client: &ClientGeneratorConf) -> String {
        todo!()
    }

    async fn generate_module_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_package_files(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_main(&self, graph: &Graph, client: &ClientGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        todo!()
    }
}
