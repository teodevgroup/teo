use crate::connectors::sql::query_builder::structs::index::SQLIndexColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLCreateIndexOnStatement {
    unique: bool,
    index: String,
    table: String,
    columns: Vec<SQLIndexColumn>
}

impl SQLCreateIndexOnStatement {
    pub fn column(&mut self, column: SQLIndexColumn) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn columns(&mut self, columns: Vec<SQLIndexColumn>) -> &mut Self {
        self.columns.extend(columns);
        self
    }
}

impl ToSQLString for SQLCreateIndexOnStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let unique = if self.unique { " UNIQUE" } else { "" };
        let index = &self.index;
        let table = &self.table;
        let def = self.columns.iter().map(|c| c.to_string(dialect)).collect::<Vec<String>>().join(", ");
        format!("CREATE{unique} INDEX `{index}` ON `{table}`({def})")
    }
}

pub struct SQLCreateIndexStatement {
    pub(crate) unique: bool,
    pub(crate) index: String,
}

impl SQLCreateIndexStatement {
    pub fn on(&self, table: impl Into<String>) -> SQLCreateIndexOnStatement {
        SQLCreateIndexOnStatement { unique: self.unique, index: self.index.clone(), table: table.into(), columns: vec![] }
    }
}
