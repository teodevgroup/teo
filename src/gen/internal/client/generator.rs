use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::file_util::FileUtil;
use crate::core::result::Result;

#[async_trait]
pub(in crate::gen) trait Generator {
    fn module_directory_in_package(&self, conf: &Conf) -> String;
    async fn generate_module_files(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()>;
    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()>;
    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()>;
}
