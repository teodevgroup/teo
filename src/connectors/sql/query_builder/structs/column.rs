use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::core::db_type::DatabaseType;

#[derive(PartialEq, Clone)]
pub struct SQLColumn {
    pub(crate) name: String,
    pub(crate) column_type: DatabaseType,
    pub(crate) not_null: bool,
    pub(crate) auto_increment: bool,
    pub(crate) default: Option<String>,
    pub(crate) primary_key: bool,
    pub(crate) unique_key: bool,
    pub(crate) extras: Vec<String>
}

impl SQLColumn {
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

impl ToSQLString for SQLColumn {
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
