use crate::gen::internal::file_util::FileUtil;

use async_trait::async_trait;

use crate::core::result::Result;
use crate::gen::internal::server::ctx::Ctx;
use crate::gen::internal::server::generator::EntityGenerator;

pub(crate) struct GoEntityGenerator {}

impl GoEntityGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EntityGenerator for GoEntityGenerator {
    async fn generate_entity_files(&self, _ctx: &Ctx, _generator: &FileUtil) -> Result<()> {
        Ok(())
    }
}
