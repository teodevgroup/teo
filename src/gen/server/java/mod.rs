use crate::core::app::conf::EntityGeneratorConf;
use crate::gen::lib::generator::Generator;
use crate::gen::server::EntityGenerator;
use crate::prelude::Graph;
use async_trait::async_trait;

pub(crate) struct JavaEntityGenerator {}

impl JavaEntityGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EntityGenerator for JavaEntityGenerator {
    async fn generate_entity_files(&self, _graph: &Graph, _conf: &EntityGeneratorConf, _generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }
}
