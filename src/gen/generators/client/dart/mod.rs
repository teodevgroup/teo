use async_trait::async_trait;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::file_util::FileUtil;

use crate::prelude::Graph;

pub(crate) struct DartClientGenerator { }

impl DartClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for DartClientGenerator {
    fn module_directory_in_package(&self, _client: &ClientGeneratorConf) -> String {
        todo!()
    }

    async fn generate_module_files(&self, _ctx: Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_package_files(&self, _ctx: Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }

    async fn generate_main(&self, _ctx: Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        todo!()
    }
}
