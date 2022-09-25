use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub struct SQLShowIndexFromStatement {
    pub(crate) table: String
}

impl ToSQLString for SQLShowIndexFromStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("SHOW INDEX FROM {table}")
    }
}
