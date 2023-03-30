use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::gen::generators::client::ClientGenerator;
use crate::gen::lib::generator::Generator;

use crate::prelude::Graph;

pub(crate) struct DartClientGenerator { }

impl DartClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for DartClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        todo!()
    }

    async fn generate_module_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_main(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        todo!()
    }
}
