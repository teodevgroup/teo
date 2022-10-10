use crate::connectors::sql::schema::dialect::SQLDialect;

pub struct SQLDescribeStatement {
    pub(crate) table: String
}

impl ToSQLString for SQLDescribeStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("DESCRIBE `{table}`")
    }
}
