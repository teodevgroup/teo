use async_trait::async_trait;
use crate::gen::interface::server::conf::Conf;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::server::generator::EntityGenerator;
use crate::prelude::Graph;



pub(in crate::gen) struct RustEntityGenerator { }

impl RustEntityGenerator {
    pub(in crate::gen) fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl EntityGenerator for RustEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &Conf, generator: &FileUtil) -> crate::prelude::Result<()> {
        todo!()
    }
}