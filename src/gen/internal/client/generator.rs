use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::file_util::FileUtil;

#[async_trait]
pub(crate) trait Generator {
    fn module_directory_in_package(&self, conf: &Conf) -> String;
    async fn generate_module_files(&self, ctx: &GenCtx, generator: &FileUtil) -> std::io::Result<()>;
    async fn generate_package_files(&self, ctx: &GenCtx, generator: &FileUtil) -> std::io::Result<()>;
    async fn generate_main(&self, ctx: &GenCtx, generator: &FileUtil) -> std::io::Result<()>;
}
