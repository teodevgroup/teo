use async_trait::async_trait;

use crate::gen::internal::file_util::FileUtil;

use crate::core::result::Result;
use crate::gen::internal::server::ctx::Ctx;

#[async_trait]
pub(in crate::gen) trait EntityGenerator {
    async fn generate_entity_files(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()>;
}
