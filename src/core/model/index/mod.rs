use crate::core::field::Sort;

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
    pub(self) name: String,
    pub(self) items: Vec<ModelIndexItem>
}

impl ModelIndex {
    pub(crate) fn r#type(&self) -> ModelIndexType {
        self.index_type
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn items(&self) -> &Vec<ModelIndexItem> {
        &self.items
    }
}
