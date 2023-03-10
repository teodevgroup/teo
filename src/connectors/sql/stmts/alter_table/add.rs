use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub struct SQLAlterTableAddStatement {
    pub(crate) table: String,
    pub(crate) column_def: SQLColumn,
}

impl ToSQLString for SQLAlterTableAddStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column_def.to_string(dialect);
        let escape = if dialect == SQLDialect::PostgreSQL { "\"" } else { "`" };
        format!("ALTER TABLE {escape}{table}{escape} ADD {def}")
    }
}
