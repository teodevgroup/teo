use crate::connectors::sql::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLUseDatabaseStatement {
    pub(crate) database: String
}

impl ToSQLString for SQLUseDatabaseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        format!("USE `{database}`")
    }
}
