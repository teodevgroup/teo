use crate::core::conf::builder::ConfBuilder;
use crate::core::database::name::DatabaseName;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::parser::Parser;
use crate::prelude::App;

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

    pub fn load(&mut self, schema_file_name: Option<&str>) {
        let mut parser = Parser::new();
        parser.parse(schema_file_name);
        self.load_config_from_parser(parser);
    }

    pub fn graph_builder(&mut self) -> &mut GraphBuilder {
        &mut self.graph_builder
    }

    pub fn conf_builder(&mut self) -> &mut ConfBuilder {
        &mut self.conf_builder
    }

    pub async fn build(&self) -> App {
        App { conf: self.conf_builder.build(), graph: self.graph_builder.build().await }
    }

    fn load_config_from_parser(&mut self, parser: &Parser) {
        // use fake conf for now
        self.conf_builder.bind(("0.0.0.0", 3000));
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let source_borrow = source.borrow();
        let connector = source_borrow.get_connector(&connector_ref.1);
        let url = connector.url.as_ref().unwrap();
        match connector.provider.unwrap() {
            DatabaseName::MySQL => self.graph_builder.data_source().mysql(url),
            DatabaseName::PostgreSQL => self.graph_builder.data_source().postgres(url),
            DatabaseName::SQLite => self.graph_builder.data_source().sqlite(url),
            DatabaseName::MongoDB => self.graph_builder.data_source().mongodb(url),
        }
        // load enums

        // load models

    }
}
