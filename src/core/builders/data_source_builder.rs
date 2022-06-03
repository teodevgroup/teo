#[cfg(feature = "data-source-mongodb")]
use crate::connectors::mongodb::MongoDBConnectorBuilder;
#[cfg(feature = "data-source-mysql")]
use crate::connectors::mysql::connector_builder::MySQLConnectorBuilder;
#[cfg(feature = "data-source-postgres")]
use crate::connectors::postgres::PostgresConnectorBuilder;
use crate::core::builders::graph_builder::GraphBuilder;


pub struct DataSourceBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
}

impl<'a> DataSourceBuilder<'a> {

    #[cfg(feature = "data-source-mongodb")]
    pub fn mongodb(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(MongoDBConnectorBuilder::new(url.into())));
    }

    #[cfg(feature = "data-source-mysql")]
    pub fn mysql(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(MySQLConnectorBuilder::new(url.into())));
    }

    #[cfg(feature = "data-source-postgres")]
    pub fn postgres(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(PostgresConnectorBuilder::new(url.into())));
    }
}
