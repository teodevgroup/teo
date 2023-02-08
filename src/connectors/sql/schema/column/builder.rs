use crate::connectors::sql::schema::column::SQLColumn;
use crate::core::database::r#type::DatabaseType;

#[derive(PartialEq, Clone)]
pub(crate) struct SQLColumnBuilder {
    pub(self) name: String,
    pub(self) r#type: Option<DatabaseType>,
    pub(self) not_null: bool,
    pub(self) auto_increment: bool,
    pub(self) default: Option<String>,
    pub(self) primary_key: bool,
    pub(self) unique_key: bool,
}

impl SQLColumnBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            r#type: None,
            not_null: false,
            auto_increment: false,
            default: None,
            primary_key: false,
            unique_key: false,
        }
    }

    pub(crate) fn column_type(&mut self, column_type: DatabaseType) -> &mut Self {
        self.r#type = Some(column_type);
        self
    }

    pub(crate) fn not_null(&mut self) -> &mut Self {
        self.not_null = true;
        self
    }

    pub(crate) fn auto_increment(&mut self) -> &mut Self {
        self.auto_increment = true;
        self
    }

    pub(crate) fn default(&mut self, value: impl Into<String>) -> &mut Self {
        self.default = Some(value.into());
        self
    }

    pub(crate) fn primary_key(&mut self) -> &mut Self {
        self.primary_key = true;
        self
    }

    pub(crate) fn unique_key(&mut self) -> &mut Self {
        self.unique_key = true;
        self
    }

    pub(crate) fn build(&self) -> SQLColumn {
        SQLColumn {
            name: self.name.clone(),
            r#type: self.r#type.as_ref().unwrap().clone(),
            not_null: self.not_null,
            auto_increment: self.auto_increment,
            default: self.default.clone(),
            primary_key: self.primary_key,
            unique_key: self.unique_key,
        }
    }
}
