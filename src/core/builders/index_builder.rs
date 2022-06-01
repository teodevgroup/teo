use crate::core::field::{IndexSettings, Sort};


pub struct IndexBuilder {
    name: Option<String>,
    sort: Sort,
    len: Option<usize>,
}

impl IndexBuilder {
    pub(crate) fn new() -> Self {
        IndexBuilder {
            name: None,
            sort: Sort::Asc,
            len: None
        }
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn asc(&mut self) -> &mut Self {
        self.sort = Sort::Asc;
        self
    }

    pub fn desc(&mut self) -> &mut Self {
        self.sort = Sort::Desc;
        self
    }

    pub fn length(&mut self, len: usize) -> &mut Self {
        self.len = Some(len);
        self
    }

    pub(crate) fn build(&mut self) -> IndexSettings {
        IndexSettings {
            name: self.name.clone(),
            sort: self.sort,
            length: self.len
        }
    }
}
