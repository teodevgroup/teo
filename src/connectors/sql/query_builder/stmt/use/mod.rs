use crate::connectors::sql::query_builder::stmt::r#use::database::SQLUseDatabaseStatement;

pub mod database;

pub struct SQLUseStatement { }

impl SQLUseStatement {
    pub fn database(&self, database: impl Into<String>) -> SQLUseDatabaseStatement {
        SQLUseDatabaseStatement { database: database.into() }
    }
}
