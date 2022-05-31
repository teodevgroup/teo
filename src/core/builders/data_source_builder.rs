use crate::connectors::mongodb::MongoDBConnectorBuilder;
use crate::connectors::mysql::MySQLConnectorBuilder;
use crate::connectors::postgres::PostgresConnectorBuilder;
use crate::core::builders::graph_builder::GraphBuilder;


pub struct DataSourceBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
}

impl<'a> DataSourceBuilder<'a> {

    pub fn mongodb(&mut self, url: &'static str) {
        self.graph_builder.connector_builder = Some(Box::new(MongoDBConnectorBuilder::new(url)));
    }

    pub fn mysql(&mut self, url: &'static str) {
        self.graph_builder.connector_builder = Some(Box::new(MySQLConnectorBuilder::new(url)));
    }

    pub fn postgres(&mut self, url: &'static str) {
        self.graph_builder.connector_builder = Some(Box::new(PostgresConnectorBuilder::new(url)));
    }
}
