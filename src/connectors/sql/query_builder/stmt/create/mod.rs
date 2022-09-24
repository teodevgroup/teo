use crate::connectors::sql::query_builder::stmt::create::database::SQLCreateDatabaseStatement;
use crate::connectors::sql::query_builder::stmt::create::index::SQLCreateIndexStatement;
use crate::connectors::sql::query_builder::stmt::create::table::SQLCreateTableStatement;

pub mod database;
pub mod table;
pub mod index;

pub struct SQLCreateStatement { }

impl SQLCreateStatement {

    pub fn database(&self, database: impl Into<String>) -> SQLCreateDatabaseStatement {
        SQLCreateDatabaseStatement { database: database.into(), if_not_exists: false }
    }

    pub fn table(&self, table: impl Into<String>) -> SQLCreateTableStatement {
        SQLCreateTableStatement { table: table.into(), if_not_exists: false, columns: vec![] }
    }

    pub fn index(&self, index: impl Into<String>) -> SQLCreateIndexStatement {
        SQLCreateIndexStatement { unique: false, index: index.into() }
    }

    pub fn unique_index(&self, index: impl Into<String>) -> SQLCreateIndexStatement {
        SQLCreateIndexStatement { unique: true, index: index.into() }
    }
}
