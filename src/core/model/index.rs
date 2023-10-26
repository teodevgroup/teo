use std::borrow::Cow;
use std::collections::HashSet;
use array_tool::vec::Join;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::field::Sort;

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
    pub(self) field_name: &'static str,
    pub(self) sort: Sort,
    pub(self) len: Option<usize>,
}

impl ModelIndexItem {

    pub(crate) fn new(name: &'static str, sort: Sort, len: Option<usize>) -> Self {
        Self {
            field_name: name,
            sort,
            len,
        }
    }

    pub(crate) fn field_name(&self) -> &str {
        self.field_name
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
    pub(crate) items: Vec<ModelIndexItem>,
    pub(self) keys: Vec<&'static str>,
}

impl ModelIndex {

    pub(crate) fn new(r#type: ModelIndexType, name: Option<impl Into<String>>, items: Vec<ModelIndexItem>) -> Self {
        let keys: Vec<&'static str> = items.iter().map(|i| i.field_name).collect();
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

    pub(crate) fn keys(&self) -> &Vec<&'static str> {
        &self.keys
    }

    pub(crate) fn keys_set(&self) -> HashSet<&'static str> {
        self.keys.iter().map(|s| *s).collect::<HashSet<&'static str>>()
    }


    pub(crate) fn append_item(&mut self, item: ModelIndexItem) {
        let key = Box::leak(Box::new(item.field_name().to_string()));
        self.items.push(item);
        self.keys.push(key.as_str());
    }
}
