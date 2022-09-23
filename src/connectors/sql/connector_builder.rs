use async_trait::async_trait;
use crate::connectors::sql::database::SQLDatabase;
use crate::connectors::sql::inferred_types::mssql::inferred_database_type_mssql;
use crate::connectors::sql::inferred_types::mysql::inferred_database_type_mysql;
use crate::connectors::sql::inferred_types::postgresql::inferred_database_type_postgresql;
use crate::connectors::sql::inferred_types::sqlite::inferred_database_type_sqlite;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;

#[derive(Debug)]
pub(crate) struct SQLConnectorBuilder {
    database: SQLDatabase,
    url: String,
}

impl SQLConnectorBuilder {
    pub(crate) fn new(database: SQLDatabase, url: String) -> Self {
        Self {
            database,
            url,
        }
    }
}

#[async_trait]
impl ConnectorBuilder for SQLConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match self.database {
            SQLDatabase::MySQL => inferred_database_type_mysql(field_type),
            SQLDatabase::PostgreSQL => inferred_database_type_postgresql(field_type),
            SQLDatabase::MSSQL => inferred_database_type_mssql(field_type),
            SQLDatabase::SQLite => inferred_database_type_sqlite(field_type),
        }
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        Box::new(SQLConnector::new(self.database, self.url.clone(), models, reset_database).await)
    }
}
