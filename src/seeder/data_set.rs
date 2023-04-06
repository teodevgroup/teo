use crate::prelude::Value;

#[derive(Debug, Clone)]
pub(crate) struct DataSet {
    pub(crate) name: String,
    pub(crate) groups: Vec<Group>
}

#[derive(Debug, Clone)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) records: Vec<Record>,
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) name: String,
    pub(crate) value: Value,
}