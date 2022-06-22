#[derive(Debug, Clone)]
pub struct EnumChoice {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) values: Vec<String>,
}
