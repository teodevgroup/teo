use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::file_util::FileUtil;



pub(crate) struct KotlinClientGenerator { }

impl KotlinClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for KotlinClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        todo!()
    }

    async fn generate_module_files(&self, _ctx: &Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_package_files(&self, _ctx: &Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_main(&self, _ctx: &Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }
}
