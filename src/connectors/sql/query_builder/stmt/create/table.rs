use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub struct SQLCreateTableStatement {
    pub(crate) table: String,
    pub(crate) if_not_exists: bool,
    pub(crate) columns: Vec<SQLColumn>
}

impl SQLCreateTableStatement {
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    pub fn column(&mut self, def: SQLColumnDef) -> &mut Self {
        self.columns.push(def);
        self
    }

    pub fn columns(&mut self, defs: Vec<SQLColumnDef>) -> &mut Self {
        self.columns.extend(defs);
        self
    }
}

impl ToSQLString for SQLCreateTableStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let if_not_exists = if self.if_not_exists { " IF NOT EXISTS" } else { "" };
        let table_name = &self.table;
        let mut columns = self.columns.iter().map(|c| {
            c.to_string(dialect)
        }).collect::<Vec<String>>().join(", ");
        format!("CREATE TABLE{if_not_exists} `{table_name}`( {columns} );")
    }
}
