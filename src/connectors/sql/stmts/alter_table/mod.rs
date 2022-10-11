use crate::connectors::sql::stmts::alter_table::add::SQLAlterTableAddStatement;
use crate::connectors::sql::stmts::alter_table::drop_column::SQLAlterTableDropColumnStatement;
use crate::connectors::sql::stmts::alter_table::modify::SQLAlterTableModifyStatement;
use crate::connectors::sql::schema::column::SQLColumn;

pub mod add;
pub mod drop_column;
pub mod modify;

pub struct SQLAlterTableStatement {
    pub(crate) table: String
}

impl SQLAlterTableStatement {
    pub(crate) fn drop_column(&self, column: impl Into<String>) -> SQLAlterTableDropColumnStatement {
        SQLAlterTableDropColumnStatement { table: self.table.clone(), column: column.into() }
    }

    pub(crate) fn modify(&self, column: SQLColumn) -> SQLAlterTableModifyStatement {
        SQLAlterTableModifyStatement { table: self.table.clone(), column }
    }

    pub(crate) fn add(&self, column_def: SQLColumn) -> SQLAlterTableAddStatement {
        SQLAlterTableAddStatement { table: self.table.clone(), column_def }
    }
}
