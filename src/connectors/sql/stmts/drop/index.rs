use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLDropIndexOnStatement {
    pub(crate) index: String,
    pub(crate) table: String,
}

impl ToSQLString for SQLDropIndexOnStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let index = &self.index;
        let table = &self.table;
        format!("DROP INDEX `{index}` on `{table}`")
    }
}

pub struct SQLDropIndexStatement {
    pub(crate) index: String
}

impl SQLDropIndexStatement {
    pub fn on(&self, table: impl Into<String>) -> SQLDropIndexOnStatement {
        SQLDropIndexOnStatement { index: self.index.clone(), table: table.into() }
    }
}
