use crate::app::program::ProgramLang;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::generators::server::go::GoEntityGenerator;
use crate::gen::generators::server::java::JavaEntityGenerator;
use crate::gen::generators::server::nodejs::gen::NodeJSEntityGenerator;
use crate::gen::generators::server::python::PythonEntityGenerator;
use crate::gen::generators::server::rust::gen::RustEntityGenerator;
use crate::gen::interface::server::conf::Conf;
use crate::prelude::Graph;
use crate::core::result::Result;
use crate::gen::generators::client::swift::types::SwiftTypes;
use crate::gen::generators::server::rust::types::RustTypes;
use crate::gen::internal::server::ctx::Ctx;
use crate::gen::internal::server::generator::EntityGenerator;

pub(crate) async fn gen(graph: &Graph, conf: &Conf) -> Result<()> {
    match conf.provider {
        ProgramLang::Rust => generate_entity_typed(RustEntityGenerator::new(), &Ctx::build(graph, conf, RustTypes::new(), RustTypes::new())).await,
        ProgramLang::Go => generate_entity_typed(GoEntityGenerator::new(), &Ctx::build(graph, conf, RustTypes::new(), RustTypes::new())).await,
        ProgramLang::NodeJS => generate_entity_typed(NodeJSEntityGenerator::new(), &Ctx::build(graph, conf, RustTypes::new(), RustTypes::new())).await,
        ProgramLang::Python => generate_entity_typed(PythonEntityGenerator::new(), &Ctx::build(graph, conf, RustTypes::new(), RustTypes::new())).await,
        ProgramLang::Java => generate_entity_typed(JavaEntityGenerator::new(), &Ctx::build(graph, conf, RustTypes::new(), RustTypes::new())).await,
        ProgramLang::C => panic!("C entity generation is not supported. Use a high level language instead."),
    }
}

async fn generate_entity_typed<T: EntityGenerator>(entity_generator: T, ctx: &Ctx<'_>) -> Result<()> {
    let dest = &ctx.conf.dest;
    let generator = FileUtil::new(&dest);
    generator.ensure_root_directory().await?;
    entity_generator.generate_entity_files(ctx, &generator).await?;
    Ok(())
}

