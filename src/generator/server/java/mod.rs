use crate::core::app::conf::EntityGeneratorConf;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
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
