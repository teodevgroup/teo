use array_tool::vec::Join;
use itertools::Itertools;
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

    pub(crate) fn sql_name(&self) -> &str {
        self.name.as_ref().unwrap().as_str()
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
}
