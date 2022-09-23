#[derive(Debug, Copy, Clone)]
pub(crate) enum SQLDatabase {
    MySQL,
    PostgreSQL,
    MSSQL,
    SQLite,
}
