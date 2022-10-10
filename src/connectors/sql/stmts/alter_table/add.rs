use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;

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
