#[cfg(feature = "mongo")]
use teo_column_type::mongo;
#[cfg(feature = "mysql")]
use teo_column_type::mysql;
#[cfg(feature = "postgres")]
use teo_column_type::postgres;
#[cfg(feature = "sqlite")]
use teo_column_type::sqlite;
use super::super::migration::TableDef;

pub trait Entity {

    #[cfg(feature = "mysql")]
    fn mongo_table_def() -> TableDef<mongo::ColumnType>;

    #[cfg(feature = "mongo")]
    fn mysql_table_def() -> TableDef<mysql::ColumnType>;

    #[cfg(feature = "postgres")]
    fn postgres_table_def() -> TableDef<postgres::ColumnType>;

    #[cfg(feature = "sqlite")]
    fn sqlite_table_def() -> TableDef<sqlite::ColumnType>;
}
