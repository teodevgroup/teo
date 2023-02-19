mod rust;
mod go;
mod java;
mod nodejs;
mod python;

use async_trait::async_trait;
use crate::core::app::conf::EntityGeneratorConf;
use crate::core::app::environment::Environment;
use crate::generator::lib::generator::Generator;
use crate::generator::lib::path::relative_to_absolute;
use crate::generator::server::go::GoEntityGenerator;
use crate::generator::server::java::JavaEntityGenerator;
use crate::generator::server::nodejs::NodeJSEntityGenerator;
use crate::generator::server::python::PythonEntityGenerator;
use crate::generator::server::rust::RustEntityGenerator;
use crate::prelude::Graph;

pub(crate) async fn generate_entity(graph: &Graph, conf: &EntityGeneratorConf) -> std::io::Result<()> {
    match conf.provider {
        Environment::Rust => generate_entity_typed(RustEntityGenerator::new(), graph, conf).await,
        Environment::Go => generate_entity_typed(GoEntityGenerator::new(), graph, conf).await,
        Environment::NodeJS => generate_entity_typed(NodeJSEntityGenerator::new(), graph, conf).await,
        Environment::Python => generate_entity_typed(PythonEntityGenerator::new(), graph, conf).await,
        Environment::Java => generate_entity_typed(JavaEntityGenerator::new(), graph, conf).await,
        Environment::C => panic!("C entity generation is not supported. Use a high level language instead."),
    }
}

pub(crate) async fn generate_entity_typed<T: EntityGenerator>(entity_generator: T, graph: &Graph, conf: &EntityGeneratorConf) -> std::io::Result<()> {
    let dest = relative_to_absolute(&conf.dest);
    let generator = Generator::new(&dest);
    generator.ensure_directory(dest).await?;
    entity_generator.generate_entity_files(graph, conf, &generator).await?;
    Ok(())
}

#[async_trait]
pub(crate) trait EntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()>;
}
