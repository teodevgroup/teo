use crate::core::database_type::DatabaseType;


pub struct SQLColumnDef {
    name: String,
    column_type: DatabaseType,
    not_null: bool,
    auto_increment: bool,
    default: Option<String>,
    primary_key: bool,
    unique_key: bool,
    extras: Vec<String>
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
        let columns = self.columns.iter().map(|c| {
            let name = &c.name;
            let t = c.column_type.to_string(dialect.clone());
            let not_null = if c.not_null { " NOT NULL" } else { " NULL" };
            let primary = if c.primary_key { " PRIMARY KEY" } else { "" };
            let auto_inc = if c.auto_increment { " AUTO INCREMENT" } else { "" };
            let unique = if c.unique_key { " UNIQUE KEY" } else { "" };
            format!("{name} {t}{not_null}{primary}{unique}{auto_inc}")
        }).collect::<Vec<String>>().join(", ");

        format!("CREATE TABLE{if_not_exists} `{table_name}` ( {columns} );")
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

pub struct SQLUseDatabaseStatement {
    database: String
}

impl ToSQLString for SQLUseDatabaseStatement {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let database = &self.database;
        format!("USE `{database}`")
    }
}

pub struct SQLUseStatement { }

impl SQLUseStatement {
    pub fn database(&self, database: impl Into<String>) -> SQLUseDatabaseStatement {
        SQLUseDatabaseStatement { database: database.into() }
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

    pub fn r#use() -> SQLUseStatement {
        SQLUseStatement { }
    }
}

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}
