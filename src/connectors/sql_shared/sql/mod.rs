use crate::core::database_type::DatabaseType;
use crate::core::field::{Field, Optionality};


#[derive(PartialEq, Clone)]
pub struct SQLColumnDef {
    pub(crate) name: String,
    pub(crate) column_type: DatabaseType,
    pub(crate) not_null: bool,
    pub(crate) auto_increment: bool,
    pub(crate) default: Option<String>,
    pub(crate) primary_key: bool,
    pub(crate) unique_key: bool,
    pub(crate) extras: Vec<String>
}

impl SQLColumnDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            column_type: DatabaseType::Undefined,
            not_null: false,
            auto_increment: false,
            default: None,
            primary_key: false,
            unique_key: false,
            extras: Vec::new()
        }
    }

    pub fn column_type(&mut self, column_type: DatabaseType) -> &mut Self {
        self.column_type = column_type;
        self
    }

    pub fn not_null(&mut self) -> &mut Self {
        self.not_null = true;
        self
    }

    pub fn auto_increment(&mut self) -> &mut Self {
        self.auto_increment = true;
        self
    }

    pub fn default(&mut self, value: impl Into<String>) -> &mut Self {
        self.default = Some(value.into());
        self
    }

    pub fn primary_key(&mut self) -> &mut Self {
        self.primary_key = true;
        self
    }

    pub fn unique_key(&mut self) -> &mut Self {
        self.unique_key = true;
        self
    }
}

impl ToSQLString for SQLColumnDef {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let name = &self.name;
        let t = self.column_type.to_string(dialect.clone());
        let not_null = if self.not_null { " NOT NULL" } else { " NULL" };
        let primary = if self.primary_key { " PRIMARY KEY" } else { "" };
        let auto_inc = if self.auto_increment { " AUTO_INCREMENT" } else { "" };
        let unique = if self.unique_key { " UNIQUE KEY" } else { "" };
        format!("`{name}` {t}{not_null}{primary}{unique}{auto_inc}")
    }
}

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

pub struct SQLCreateDatabaseStatement {
    database: String,
    if_not_exists: bool,
}

impl SQLCreateDatabaseStatement {
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }
}

impl ToSQLString for SQLCreateDatabaseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        let if_not_exists = if self.if_not_exists { " IF NOT EXISTS" } else { "" };
        format!("CREATE DATABASE{if_not_exists} `{database}`;")
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

pub struct SQLCreateTableStatement {
    table: String,
    if_not_exists: bool,
    columns: Vec<SQLColumnDef>
}

impl SQLCreateTableStatement {
    pub fn if_not_exists(&mut self) -> &mut Self {
        self.if_not_exists = true;
        self
    }

    pub fn column(&mut self, def: SQLColumnDef) -> &mut Self {
        self.columns.push(def);
        self
    }

    pub fn columns(&mut self, defs: Vec<SQLColumnDef>) -> &mut Self {
        self.columns.extend(defs);
        self
    }
}

impl ToSQLString for SQLCreateTableStatement {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let if_not_exists = if self.if_not_exists { " IF NOT EXISTS" } else { "" };
        let table_name = &self.table;
        let mut columns = self.columns.iter().map(|c| {
            c.to_string(dialect)
        }).collect::<Vec<String>>().join(", ");
        format!("CREATE TABLE{if_not_exists} `{table_name}`( {columns} );")
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

pub struct SQLCreateStatement { }

impl SQLCreateStatement {

    pub fn database(&self, database: impl Into<String>) -> SQLCreateDatabaseStatement {
        SQLCreateDatabaseStatement { database: database.into(), if_not_exists: false }
    }

    pub fn table(&self, table: impl Into<String>) -> SQLCreateTableStatement {
        SQLCreateTableStatement { table: table.into(), if_not_exists: false, columns: vec![] }
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
}

pub struct SQLUseStatement {
    database: String
}

impl ToSQLString for SQLUseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        format!("USE `{database}`")
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

#[derive(PartialEq, Copy, Clone)]
pub enum SQLDialect {
    MySQL,
    PostgreSQL,
    SQLite,
    MSSQL,
}

pub struct SQL { }

impl SQL {
    pub fn create() -> SQLCreateStatement {
        SQLCreateStatement { }
    }

    pub fn drop() -> SQLDropStatement {
        SQLDropStatement { }
    }

    pub fn r#use(database: impl Into<String>) -> SQLUseStatement {
        SQLUseStatement { database: database.into() }
    }

    pub fn show() -> SQLShowStatement { SQLShowStatement { } }

    pub fn describe(table: impl Into<String>) -> SQLDescribeStatement {
        SQLDescribeStatement { table: table.into() }
    }

    pub fn alter_table(table: impl Into<String>) -> SQLAlterTableStatement {
        SQLAlterTableStatement { table: table.into() }
    }
}

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}
