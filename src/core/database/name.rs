#[derive(Debug, Clone, Copy)]
pub enum DatabaseName {
    MySQL,
    PostgreSQL,
    #[cfg(feature = "data-source-sqlite")]
    SQLite,
    MongoDB,
}
