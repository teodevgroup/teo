use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub struct SQLAlterTableAddStatement {
    pub(crate) table: String,
    pub(crate) column_def: SQLColumn,
}

impl ToSQLString for SQLAlterTableAddStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column_def.to_string(dialect);
        format!("ALTER TABLE `{table}` ADD {def}")
    }
}
