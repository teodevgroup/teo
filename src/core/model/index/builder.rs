use crate::core::field::Sort;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};

pub struct ModelIndexBuilder {
    index_type: ModelIndexType,
    name: Option<String>,
    items: Vec<ModelIndexItem>,
}

impl ModelIndexBuilder {
    pub(crate) fn new(index_type: ModelIndexType) -> Self {
        ModelIndexBuilder {
            index_type,
            name: None,
            items: Vec::new(),
        }
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn field(&mut self, name: impl Into<String>) -> &mut Self {
        self.items.push(ModelIndexItem {
            field_name: name.into(),
            sort: Sort::Asc,
            len: None
        });
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.items.last_mut().unwrap().sort = Sort::Asc;
        self
    }

    pub fn desc(&mut self) -> &mut Self {
        self.items.last_mut().unwrap().sort = Sort::Desc;
        self
    }

    pub fn length(&mut self, len: usize) -> &mut Self {
        self.items.last_mut().unwrap().len = Some(len);
        self
    }

    pub(crate) fn build(&mut self) -> ModelIndex {
        ModelIndex {
            index_type: self.index_type,
            name: self.name.clone().unwrap(),
            items: self.items.clone(),
            keys: self.items.iter().map(|i| i.field_name.to_owned()).collect(),
        }
    }
}
