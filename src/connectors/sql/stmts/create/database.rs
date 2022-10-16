use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub struct SQLCreateDatabaseStatement {
    pub(crate) database: String,
    pub(crate) if_not_exists: bool,
}

impl SQLCreateDatabaseStatement {
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }
}

impl ToSQLString for SQLCreateDatabaseStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let database = &self.database;
        let if_not_exists = if self.if_not_exists { " IF NOT EXISTS" } else { "" };
        if dialect == SQLDialect::PostgreSQL {
            format!("CREATE DATABASE{if_not_exists} {database};")
        } else {
            format!("CREATE DATABASE{if_not_exists} `{database}`;")
        }
    }
}
