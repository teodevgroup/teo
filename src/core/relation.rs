#[derive(Debug, Clone)]
pub(crate) struct Relation {
    pub(crate) name: String,
    pub(crate) model: String,
    pub(crate) is_vec: bool,
    pub(crate) fields: Vec<String>,
    pub(crate) references: Vec<String>,
}