use crate::core::conf::builder::ConfBuilder;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::parser::Parser;

pub struct AppBuilder {
    pub(crate) graph_builder: GraphBuilder,
    pub(crate) conf_builder: ConfBuilder,
}

impl AppBuilder {

    pub fn new() -> Self {
        Self {
            graph_builder: GraphBuilder::new(),
            conf_builder: ConfBuilder::new(),
        }
    }

    pub fn load(&mut self, schema: Option<&str>) {
        let mut parser = Parser::new();
        parser.parse(schema);
    }

    pub fn graph_builder(&mut self) -> &mut GraphBuilder {
        &mut self.graph_builder
    }

    pub fn conf_builder(&mut self) -> &mut ConfBuilder {
        &mut self.conf_builder
    }


}
