use async_trait::async_trait;
use crate::core::app::conf::ClientGeneratorConf;
use crate::gen::generators::client::ClientGenerator;
use crate::gen::lib::file_util::FileUtil;

use crate::prelude::Graph;

pub(crate) struct KotlinClientGenerator { }

impl KotlinClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl ClientGenerator for KotlinClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        todo!()
    }

    async fn generate_module_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_package_files(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_main(&self, _graph: &Graph, _client: &ClientGeneratorConf, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }
}
