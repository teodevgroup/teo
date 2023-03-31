use async_trait::async_trait;
use crate::core::graph::Graph;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::file_util::FileUtil;

pub(crate) struct CSharpClientGenerator { }

impl CSharpClientGenerator {
    pub(crate) fn new() -> Self { Self {} }
}

#[async_trait]
impl Generator for CSharpClientGenerator {
    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        generator.ensure_root_directory().await?;
        generator.clear_root_directory().await?;
        Ok(())
        //generator.generate_file("Runtime.cs", generate_runtime_cs(ctx.graph, ctx.conf).await).await
    }

    async fn generate_package_files(&self, _ctx: &Ctx, _generator: &FileUtil) -> std::io::Result<()> {
        Ok(())
    }

    async fn generate_main(&self, _ctx: &Ctx, generator: &FileUtil) -> std::io::Result<()> {
        Ok(())
        //generator.generate_file("Index.cs", generate_index_cs(graph, client).await).await
    }
}
