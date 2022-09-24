use std::sync::Arc;
use crate::core::db_type::DatabaseType;
use crate::core::field::{Field, Optionality};
use crate::core::model::Model;

pub mod dialect;
pub mod column;
pub mod stmt;
pub mod traits;
pub mod structs;

impl From<&Field> for SQLColumnDef {
    fn from(field: &Field) -> Self {
        let mut column = SQLColumnDef::new(field.column_name());
        column.column_type(field.database_type.clone());
        match field.optionality {
            Optionality::Required => {
                column.not_null();
            }
            Optionality::Optional => {}
        }
        if field.primary {
            column.primary_key();
        }
        if field.auto_increment {
            column.auto_increment();
        }
        column
    }
}

impl From<&Arc<Field>> for SQLColumnDef {
    fn from(field: &Arc<Field>) -> Self {
        let mut column = SQLColumnDef::new(field.column_name());
        column.column_type(field.database_type.clone());
        match field.optionality {
            Optionality::Required => {
                column.not_null();
            }
            Optionality::Optional => {}
        }
        if field.primary {
            column.primary_key();
        }
        if field.auto_increment {
            column.auto_increment();
        }
        column
    }
}



pub struct SQLDropDatabaseStatement {
    database: String,
    if_exists: bool,
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

pub struct SQLDropTableStatement {
    table: String,
    if_exists: bool,
}

impl SQLDropTableStatement {
    pub fn if_exists(&mut self) -> &mut Self {
        self.if_exists = true;
        self
    }
}

impl ToSQLString for SQLDropTableStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        let if_exists = if self.if_exists { " IF EXISTS" } else { "" };
        format!("DROP TABLE{if_exists} `{table}`;")
    }
}

pub struct SQLDropIndexOnStatement {
    index: String,
    table: String,
}

impl ToSQLString for SQLDropIndexOnStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let index = &self.index;
        let table = &self.table;
        format!("DROP INDEX `{index}` on `{table}`")
    }
}

pub struct SQLDropIndexStatement {
    index: String
}

impl SQLDropIndexStatement {
    pub fn on(&self, table: impl Into<String>) -> SQLDropIndexOnStatement {
        SQLDropIndexOnStatement { index: self.index.clone(), table: table.into() }
    }
}

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

pub struct SQLUseStatement { }

pub struct SQLUseDatabaseStatement {
    database: String
}

impl ToSQLString for SQLUseDatabaseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        format!("USE `{database}`")
    }
}

impl SQLUseStatement {
    pub fn database(&self, database: impl Into<String>) -> SQLUseDatabaseStatement {
        SQLUseDatabaseStatement { database: database.into() }
    }
}

pub struct SQLShowTablesStatement {
    like: Option<String>
}

impl SQLShowTablesStatement {
    pub fn like(&mut self, name: impl Into<String>) -> &mut Self {
        self.like = Some(name.into());
        self
    }
}

impl ToSQLString for SQLShowTablesStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let like = match &self.like {
            Some(name) => format!(" like \"{name}\""),
            None => "".to_string()
        };
        format!("SHOW TABLES{like}")
    }
}

pub struct SQLShowIndexFromStatement {
    table: String
}

impl ToSQLString for SQLShowIndexFromStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("SHOW INDEX FROM {table}")
    }
}

pub struct SQLShowStatement { }

impl SQLShowStatement {
    pub fn tables(&self) -> SQLShowTablesStatement {
        SQLShowTablesStatement { like: None }
    }

    pub fn index_from(&self, table: impl Into<String>) -> SQLShowIndexFromStatement {
        SQLShowIndexFromStatement { table: table.into() }
    }
}

pub struct SQLDescribeStatement {
    table: String
}

impl ToSQLString for SQLDescribeStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        format!("DESCRIBE `{table}`")
    }
}

pub struct SQLAlterTableStatement {
    table: String
}

impl SQLAlterTableStatement {
    pub fn drop_column(&self, column: impl Into<String>) -> SQLAlterTableDropColumnStatement {
        SQLAlterTableDropColumnStatement { table: self.table.clone(), column: column.into() }
    }

    pub fn modify(&self, column_def: SQLColumnDef) -> SQLAlterTableModifyStatement {
        SQLAlterTableModifyStatement { table: self.table.clone(), column_def }
    }

    pub fn add(&self, column_def: SQLColumnDef) -> SQLAlterTableAddStatement {
        SQLAlterTableAddStatement { table: self.table.clone(), column_def }
    }
}

pub struct SQLAlterTableAddStatement {
    table: String,
    column_def: SQLColumnDef,
}

impl ToSQLString for SQLAlterTableAddStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column_def.to_string(dialect);
        format!("ALTER TABLE `{table}` ADD {def}")
    }
}

pub struct SQLAlterTableModifyStatement {
    table: String,
    column_def: SQLColumnDef,
}

impl ToSQLString for SQLAlterTableModifyStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let table = &self.table;
        let def = self.column_def.to_string(dialect);
        format!("ALTER TABLE `{table}` MODIFY {def}")
    }
}

pub struct SQLAlterTableDropColumnStatement {
    table: String,
    column: String,
}

impl ToSQLString for SQLAlterTableDropColumnStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let table = &self.table;
        let column = &self.column;
        format!("ALTER TABLE `{table}` DROP COLUMN `{column}`")
    }
}

pub struct SQL { }

impl SQL {
    pub fn create() -> SQLCreateStatement {
        SQLCreateStatement { }
    }

    pub fn drop() -> SQLDropStatement {
        SQLDropStatement { }
    }

    pub fn r#use() -> SQLUseStatement {
        SQLUseStatement { }
    }

    pub fn show() -> SQLShowStatement { SQLShowStatement { } }

    pub fn describe(table: impl Into<String>) -> SQLDescribeStatement {
        SQLDescribeStatement { table: table.into() }
    }

    pub fn alter_table(table: impl Into<String>) -> SQLAlterTableStatement {
        SQLAlterTableStatement { table: table.into() }
    }
}

pub(crate) fn table_create_statement(model: &Model) -> SQLCreateTableStatement {
    let mut stmt = SQL::create().table(model.table_name());
    stmt.if_not_exists();
    for field in model.fields() {
        stmt.column(field.into());
    }
    stmt
}
