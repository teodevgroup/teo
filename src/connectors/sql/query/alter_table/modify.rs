use crate::connectors::sql::dialect::SQLDialect;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLAlterTableModifyStatement {
    pub(crate) table: String,
    pub(crate) column_def: SQLColumn,
}

impl ToSQLString for SQLAlterTableModifyStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column_def.to_string(dialect);
        format!("ALTER TABLE `{table}` MODIFY {def}")
    }
}
