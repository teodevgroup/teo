use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLDropDatabaseStatement {
    pub(crate) database: String,
    pub(crate) if_exists: bool,
}

impl SQLDropDatabaseStatement {
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }
}

impl ToSQLString for SQLDropDatabaseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        let if_exists = if self.if_exists { " IF EXISTS" } else { "" };
        format!("DROP DATABASE{if_exists} `{database}`;")
    }
}
