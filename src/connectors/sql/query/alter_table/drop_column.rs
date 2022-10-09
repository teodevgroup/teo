use crate::connectors::sql::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLAlterTableDropColumnStatement {
    pub(crate) table: String,
    pub(crate) column: String,
}

impl ToSQLString for SQLAlterTableDropColumnStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        let column = &self.column;
        format!("ALTER TABLE `{table}` DROP COLUMN `{column}`")
    }
}
