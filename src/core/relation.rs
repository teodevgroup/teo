use crate::core::object::Object;

#[derive(Debug, Clone)]
pub(crate) struct Relation {
    pub(crate) name: String,
    pub(crate) model: String,
    pub(crate) through: Option<String>,
    pub(crate) is_vec: bool,
    pub(crate) fields: Vec<String>,
    pub(crate) references: Vec<String>,
}

pub(crate) enum RelationManipulation {
    Connect(Object),
    Disconnect(Object),
    Set(Object),
    Keep(Object),
}