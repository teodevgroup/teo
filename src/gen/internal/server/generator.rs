use async_trait::async_trait;
use crate::gen::interface::server::conf::Conf;
use crate::gen::internal::file_util::FileUtil;
use crate::prelude::Graph;
use crate::core::result::Result;

#[async_trait]
pub(in crate::gen) trait EntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &Conf, generator: &FileUtil) -> Result<()>;
}
