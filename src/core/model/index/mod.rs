use std::borrow::Cow;
use array_tool::vec::Join;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::Sort;

pub mod builder;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

    pub(crate) fn sql_name(&self, table_name: &str) -> Cow<str> {
        if self.name.is_some() {
            Cow::Borrowed(self.name.as_ref().unwrap().as_str())
        } else {
            Cow::Owned(self.normalize_name(table_name))
        }
    }

    pub(crate) fn mongodb_name(&self) -> String {
        if self.name.is_some() {
            self.name().unwrap().to_owned()
        } else {
            let mut keys = self.keys.clone();
            keys.sort();
            keys.join("_")
        }
    }

    pub(crate) fn items(&self) -> &Vec<ModelIndexItem> {
        &self.items
    }

    pub(crate) fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub(crate) fn to_sql_create(&self, dialect: SQLDialect, table_name: &str) -> String {
        let escape = dialect.escape();
        let index_name_cow = self.sql_name(table_name);
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

    pub(crate) fn normalize_name(&self, table_name: &str) -> String {
        format!("{table_name}_{}_{}", self.ordered_names(), self.psql_suffix())
    }

    fn psql_suffix(&self) -> &str {
        if self.index_type.is_primary() {
            "pkey"
        } else {
            "idx"
        }
    }

    fn ordered_names(&self) -> String {
        let mut keys = self.keys.clone();
        keys.sort();
        keys.join("_")
    }
}
