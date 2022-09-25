use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub struct SQLDescribeStatement {
    pub(crate) table: String
}

impl ToSQLString for SQLDescribeStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("DESCRIBE `{table}`")
    }
}
