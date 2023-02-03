use crate::core::app::conf::EntityGeneratorConf;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
use crate::prelude::Graph;
use async_trait::async_trait;

pub(crate) struct NodeJSEntityGenerator {}

impl NodeJSEntityGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EntityGenerator for NodeJSEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }
}
