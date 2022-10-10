use crate::connectors::sql::stmts::drop::database::SQLDropDatabaseStatement;
use crate::connectors::sql::stmts::drop::index::SQLDropIndexStatement;
use crate::connectors::sql::stmts::drop::table::SQLDropTableStatement;

pub mod database;
pub mod table;
pub mod index;

pub struct SQLDropStatement { }

impl SQLDropStatement {

    pub fn database(&self, database: impl Into<String>) -> SQLDropDatabaseStatement {
        SQLDropDatabaseStatement { database: database.into(), if_exists: false }
    }

    pub fn table(&self, table: impl Into<String>) -> SQLDropTableStatement {
        SQLDropTableStatement { table: table.into(), if_exists: false }
    }

    pub fn index(&self, index: impl Into<String>) -> SQLDropIndexStatement {
        SQLDropIndexStatement { index: index.into() }
    }
}
