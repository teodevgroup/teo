use array_tool::vec::Join;
use crate::core::app::conf::EntityGeneratorConf;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
use crate::prelude::Graph;
use async_trait::async_trait;

pub(crate) struct NodeJSEntityGenerator { }

impl NodeJSEntityGenerator {
    pub fn new() -> Self {
        Self { }
    }

    async fn generate_index_js(&self, graph: &Graph, generator: &Generator) -> std::io::Result<()> {
        let names: Vec<&str> = graph.models().iter().map(|m| m.name()).collect();
        let prefixed_names: Vec<String> = names.iter().map(|n| "  ".to_owned() + n).collect();
        let import = "const { getModelClass } = require(\"@teocloud/teo\")";
        let body = names.iter().map(|n| format!("const {} = getModelClass('{}')", *n, *n)).collect::<Vec<String>>().join("\n");
        let export = format!("module.exports = {{\n{},\n}}", prefixed_names.join(",\n"));
        generator.generate_file("index.js", format!("{import}\n\n{body}\n\n{export}\n")).await
    }

    async fn generate_index_d_ts(&self, graph: &Graph, generator: &Generator) -> std::io::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl EntityGenerator for NodeJSEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, _conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        self.generate_index_js(graph, generator).await?;
        self.generate_index_d_ts(graph, generator).await?;
        Ok(())
    }
}
