use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub struct SQLAlterTableModifyStatement {
    pub(crate) table: String,
    pub(crate) column: SQLColumn,
}

impl ToSQLString for SQLAlterTableModifyStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column.to_string(dialect);
        if dialect == SQLDialect::SQLite {
            format!("ALTER TABLE `{table}` ({def})")
        } else {
            format!("ALTER TABLE `{table}` MODIFY {def}")
        }
    }
}
