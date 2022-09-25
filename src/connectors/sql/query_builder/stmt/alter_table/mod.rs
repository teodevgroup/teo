use crate::connectors::sql::query_builder::stmt::alter_table::add::SQLAlterTableAddStatement;
use crate::connectors::sql::query_builder::stmt::alter_table::drop_column::SQLAlterTableDropColumnStatement;
use crate::connectors::sql::query_builder::stmt::alter_table::modify::SQLAlterTableModifyStatement;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;

pub mod add;
pub mod drop_column;
pub mod modify;

pub struct SQLAlterTableStatement {
    pub(crate) table: String
}

impl SQLAlterTableStatement {
    pub fn drop_column(&self, column: impl Into<String>) -> SQLAlterTableDropColumnStatement {
        SQLAlterTableDropColumnStatement { table: self.table.clone(), column: column.into() }
    }

    pub fn modify(&self, column_def: SQLColumn) -> SQLAlterTableModifyStatement {
        SQLAlterTableModifyStatement { table: self.table.clone(), column_def }
    }

    pub fn add(&self, column_def: SQLColumn) -> SQLAlterTableAddStatement {
        SQLAlterTableAddStatement { table: self.table.clone(), column_def }
    }
}
