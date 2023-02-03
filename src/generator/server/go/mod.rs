use crate::core::app::conf::EntityGeneratorConf;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
use crate::prelude::Graph;
use async_trait::async_trait;

pub(crate) struct GoEntityGenerator {}

impl GoEntityGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EntityGenerator for GoEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }
}
