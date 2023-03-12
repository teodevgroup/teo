#[derive(Debug, Clone)]
pub(crate) struct ModelMigration {
    pub(crate) renamed: Vec<String>,
    pub(crate) version: Option<String>,
    pub(crate) drop: bool,
}
