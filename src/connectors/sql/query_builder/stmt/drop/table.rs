use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub struct SQLDropTableStatement {
    pub(crate) table: String,
    pub(crate) if_exists: bool,
}

impl SQLDropTableStatement {
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }
}

impl ToSQLString for SQLDropTableStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        let if_exists = if self.if_exists { " IF EXISTS" } else { "" };
        format!("DROP TABLE{if_exists} `{table}`;")
    }
}
