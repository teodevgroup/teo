use crate::connectors::sql::stmts::r#use::database::SQLUseDatabaseStatement;

pub mod database;

pub(crate) struct SQLUseStatement { }

impl SQLUseStatement {
    pub fn database(&self, database: impl Into<String>) -> SQLUseDatabaseStatement {
        SQLUseDatabaseStatement { database: database.into() }
    }
}
