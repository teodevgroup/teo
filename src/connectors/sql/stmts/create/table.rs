use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub(crate) struct SQLCreateTableStatement {
    pub(crate) table: String,
    pub(crate) if_not_exists: bool,
    pub(crate) columns: Vec<SQLColumn>
}

impl SQLCreateTableStatement {
    pub(crate) fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    pub(crate) fn column(&mut self, def: SQLColumn) -> &mut Self {
        self.columns.push(def);
        self
    }

    pub(crate) fn columns(&mut self, defs: Vec<SQLColumn>) -> &mut Self {
        self.columns.extend(defs);
        self
    }
}

impl ToSQLString for SQLCreateTableStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let if_not_exists = if self.if_not_exists { " IF NOT EXISTS" } else { "" };
        let table_name = &self.table;
        let columns = self.columns.iter().map(|c| {
            c.to_string(dialect)
        }).collect::<Vec<String>>().join(", ");
        if dialect == SQLDialect::PostgreSQL {
            format!("CREATE TABLE{if_not_exists} \"{table_name}\"( {columns} );")
        } else {
            format!("CREATE TABLE{if_not_exists} `{table_name}`( {columns} );")
        }
    }
}
