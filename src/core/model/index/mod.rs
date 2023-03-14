use std::borrow::Cow;
use array_tool::vec::Join;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::Sort;

pub mod builder;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModelIndexType {
    Primary,
    Index,
    Unique,
}

impl ModelIndexType {
    pub(crate) fn is_primary(&self) -> bool {
        match self {
            ModelIndexType::Primary => true,
            _ => false,
        }
    }

    pub(crate) fn is_unique(&self) -> bool {
        match self {
            ModelIndexType::Unique | ModelIndexType::Primary => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub(crate) struct ModelIndexItem {
    pub(self) field_name: String,
    pub(self) sort: Sort,
    pub(self) len: Option<usize>,
}

impl ModelIndexItem {

    pub(crate) fn new(name: impl Into<String>, sort: Sort, len: Option<usize>) -> Self {
        Self {
            field_name: name.into(),
            sort,
            len,
        }
    }

    pub(crate) fn field_name(&self) -> &str {
        &self.field_name
    }

    pub(crate) fn sort(&self) -> Sort {
        self.sort
    }

    pub(crate) fn len(&self) -> Option<usize> {
        self.len
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) struct ModelIndex {
    pub(self) index_type: ModelIndexType,
    pub(self) name: Option<String>,
    pub(self) items: Vec<ModelIndexItem>,
    pub(self) keys: Vec<String>,
}

impl ModelIndex {

    pub(crate) fn new(r#type: ModelIndexType, name: Option<impl Into<String>>, items: Vec<ModelIndexItem>) -> Self {
        let keys: Vec<String> = items.iter().map(|i| i.field_name.clone()).collect();
        Self {
            index_type: r#type,
            name: name.map(|v| v.into()),
            items,
            keys,
        }
    }
    pub(crate) fn r#type(&self) -> ModelIndexType {
        self.index_type
    }

    pub(crate) fn name(&self) -> Option<&str> {
        match &self.name {
            Some(n) => Some(n.as_str()),
            None => None,
        }
    }

    pub(crate) fn sql_name(&self, table_name: &str, dialect: SQLDialect) -> Cow<str> {
        if self.name.is_some() {
            Cow::Borrowed(self.name.as_ref().unwrap().as_str())
        } else {
            Cow::Owned(self.normalize_name(table_name, dialect))
        }
    }

    pub(crate) fn mongodb_name(&self) -> String {
        if self.name.is_some() {
            self.name().unwrap().to_owned()
        } else {
            self.keys.join("_")
        }
    }

    pub(crate) fn set_name(&mut self, new_name: String) {
        self.name = Some(new_name);
    }

    pub(crate) fn items(&self) -> &Vec<ModelIndexItem> {
        &self.items
    }

    pub(crate) fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub(crate) fn to_sql_drop(&self, dialect: SQLDialect, table_name: &str) -> String {
        let escape = dialect.escape();
        let index_name_cow = self.sql_name(table_name, dialect);
        let index_name = index_name_cow.as_ref();
        format!("DROP INDEX {escape}{index_name}{escape} ON {escape}{table_name}{escape}")
    }

    pub(crate) fn to_sql_create(&self, dialect: SQLDialect, table_name: &str) -> String {
        let escape = dialect.escape();
        let index_name_cow = self.sql_name(table_name, dialect);
        let index_name = index_name_cow.as_ref();
        let unique = if self.r#type().is_unique() { "UNIQUE " } else { "" };
        let fields: Vec<String> = self.items.iter().map(|item| {
            Self::sql_format_item(dialect, item)
        }).collect();
        format!("CREATE {unique}INDEX {escape}{index_name}{escape} {escape}{table_name}{escape}({})", fields.join(","))
    }

    pub(crate) fn sql_format_item(dialect: SQLDialect, item: &ModelIndexItem) -> String {
        let escape = dialect.escape();
        let name = item.field_name();
        let sort = item.sort().to_str();
        let len = if let Some(len) = item.len() {
            if dialect == SQLDialect::MySQL {
                Cow::Owned(format!("({})", len))
            } else {
                Cow::Borrowed("")
            }
        } else {
            Cow::Borrowed("")
        };
        format!("{escape}{name}{escape}{len} {sort}")
    }

    pub(crate) fn normalize_name(&self, table_name: &str, dialect: SQLDialect) -> String {
        match self.index_type {
            ModelIndexType::Primary => match dialect {
                SQLDialect::MySQL => "PRIMARY".to_owned(),
                SQLDialect::SQLite => format!("sqlite_autoindex_{}_1", table_name),
                SQLDialect::PostgreSQL => self.normalize_name_psql(table_name),
                _ => unreachable!()
            },
            _ => match dialect {
                SQLDialect::PostgreSQL => self.normalize_name_psql(table_name),
                _ => self.normalize_name_normal(table_name),
            }
        }
    }

    fn normalize_name_normal(&self, table_name: &str) -> String {
        format!("{table_name}_{}", self.joined_names())
    }

    fn normalize_name_psql(&self, table_name: &str) -> String {
        format!("{table_name}_{}_{}", self.joined_names(), self.psql_suffix())
    }

    fn psql_suffix(&self) -> &str {
        if self.index_type.is_primary() {
            "pkey"
        } else {
            "idx"
        }
    }

    fn joined_names(&self) -> String {
        self.keys.join("_")
    }

    pub(crate) fn append_item(&mut self, item: ModelIndexItem) {
        let key = item.field_name().to_owned();
        self.items.push(item);
        self.keys.push(key);
    }
}
