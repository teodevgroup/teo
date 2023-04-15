use async_trait::async_trait;
use crate::app::conf::EntityGeneratorConf;
use crate::app::environment::Environment;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::generators::server::go::GoEntityGenerator;
use crate::gen::generators::server::java::JavaEntityGenerator;
use crate::gen::generators::server::nodejs::NodeJSEntityGenerator;
use crate::gen::generators::server::python::PythonEntityGenerator;
use crate::gen::generators::server::rust::RustEntityGenerator;
use crate::prelude::Graph;

pub(crate) async fn gen(graph: &Graph, conf: &EntityGeneratorConf) -> std::io::Result<()> {
    match conf.provider {
        Environment::Rust => generate_entity_typed(RustEntityGenerator::new(), graph, conf).await,
        Environment::Go => generate_entity_typed(GoEntityGenerator::new(), graph, conf).await,
        Environment::NodeJS => generate_entity_typed(NodeJSEntityGenerator::new(), graph, conf).await,
        Environment::Python => generate_entity_typed(PythonEntityGenerator::new(), graph, conf).await,
        Environment::Java => generate_entity_typed(JavaEntityGenerator::new(), graph, conf).await,
        Environment::C => panic!("C entity generation is not supported. Use a high level language instead."),
    }
}

async fn generate_entity_typed<T: EntityGenerator>(entity_generator: T, graph: &Graph, conf: &EntityGeneratorConf) -> std::io::Result<()> {
    let dest = &conf.dest;
    let generator = FileUtil::new(&dest);
    generator.ensure_root_directory().await?;
    entity_generator.generate_entity_files(graph, conf, &generator).await?;
    Ok(())
}

#[async_trait]
pub(in crate::gen) trait EntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &EntityGeneratorConf, generator: &FileUtil) -> std::io::Result<()>;
}
