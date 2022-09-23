#[cfg(feature = "data-source-mongodb")]
use crate::connectors::mongodb::connector_builder::MongoDBConnectorBuilder;
#[cfg(any(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mssql", feature = "data-source-sqlite"))]
use crate::connectors::sql::connector_builder::SQLConnectorBuilder;
#[cfg(any(feature = "data-source-mysql", feature = "data-source-postgres", feature = "data-source-mssql", feature = "data-source-sqlite"))]
use crate::connectors::sql::query_builder::SQLDialect;
use crate::core::graph::builder::GraphBuilder;

pub struct DataSourceBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
}

impl<'a> DataSourceBuilder<'a> {

    #[cfg(feature = "data-source-mysql")]
    pub fn mysql(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(
            SQLConnectorBuilder::new(SQLDialect::MySQL, url.into())));
    }

    #[cfg(feature = "data-source-postgres")]
    pub fn postgres(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(
            SQLConnectorBuilder::new(SQLDialect::PostgreSQL, url.into())));
    }

    #[cfg(feature = "data-source-mssql")]
    pub fn mssql(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(
            SQLConnectorBuilder::new(SQLDialect::MSSQL, url.into())));
    }

    #[cfg(feature = "data-source-sqlite")]
    pub fn sqlite(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(
            SQLConnectorBuilder::new(SQLDialect::SQLite, url.into())));
    }

    #[cfg(feature = "data-source-mongodb")]
    pub fn mongodb(&mut self, url: impl Into<String>) {
        self.graph_builder.connector_builder = Some(Box::new(MongoDBConnectorBuilder::new(url.into())));
    }
}
