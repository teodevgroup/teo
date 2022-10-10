use async_trait::async_trait;
use sqlx::Database;
use crate::connectors::sql::connector::SQLConnector;
use crate::connectors::sql::inferred_types::mssql::inferred_database_type_mssql;
use crate::connectors::sql::inferred_types::mysql::inferred_database_type_mysql;
use crate::connectors::sql::inferred_types::postgresql::inferred_database_type_postgresql;
use crate::connectors::sql::inferred_types::sqlite::inferred_database_type_sqlite;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;

#[derive(Debug)]
pub(crate) struct SQLConnectorBuilder {
    dialect: SQLDialect,
    url: String,
}

impl SQLConnectorBuilder {
    pub(crate) fn new(dialect: SQLDialect, url: String) -> Self {
        Self {
            dialect,
            url,
        }
    }
}

#[async_trait]
impl ConnectorBuilder for SQLConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        field_type.to_database_type(self.dialect)
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        Box::new(SQLConnector::new(self.dialect, self.url.clone(), models, reset_database).await)
    }
}
