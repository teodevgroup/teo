use async_trait::async_trait;
use sqlx::Database;
use crate::connectors::sql::connector::SQLConnector;
use crate::connectors::sql::query_builder::SQLDialect;
use crate::connectors::sql::inferred_types::mssql::inferred_database_type_mssql;
use crate::connectors::sql::inferred_types::mysql::inferred_database_type_mysql;
use crate::connectors::sql::inferred_types::postgresql::inferred_database_type_postgresql;
use crate::connectors::sql::inferred_types::sqlite::inferred_database_type_sqlite;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;

#[derive(Debug)]
pub(crate) struct SQLConnectorBuilder<T: Database + Send + Sync> {
    database: T,
    dialect: SQLDialect,
    url: String,
}

impl<T: Database + Send + Sync> SQLConnectorBuilder<T> {
    pub(crate) fn new(database: T, dialect: SQLDialect, url: String) -> Self {
        Self {
            database,
            dialect,
            url,
        }
    }
}

#[async_trait]
impl<T: Database + Send + Sync> ConnectorBuilder for SQLConnectorBuilder<T> {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match self.dialect {
            SQLDialect::SQLite => inferred_database_type_sqlite(field_type),
            SQLDialect::MySQL => inferred_database_type_mysql(field_type),
            SQLDialect::PostgreSQL => inferred_database_type_postgresql(field_type),
            SQLDialect::MSSQL => inferred_database_type_mssql(field_type),
        }
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        Box::new(SQLConnector::<T>::new(self.url.clone(), models, reset_database).await)
    }
}
