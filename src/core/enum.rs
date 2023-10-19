#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DbEnum {
    pub(crate) choices: Vec<String>,
}
