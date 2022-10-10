use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::db_type::DatabaseType;

pub(crate) struct SQLTypeDecoder { }

impl SQLTypeDecoder {
    pub(crate) fn decode(r#type: &str, dialect: SQLDialect) -> DatabaseType {

    }
}
