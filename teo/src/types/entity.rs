#[cfg(feature = "mongo")]
use teo_column_type::MongoColumnType;
#[cfg(feature = "mysql")]
use teo_column_type::MySQLColumnType;
#[cfg(feature = "postgres")]
use teo_column_type::PostgresColumnType;
#[cfg(feature = "sqlite")]
use teo_column_type::SQLiteColumnType;
use super::super::migration::TableDef;

pub trait Entity {

    #[cfg(feature = "mysql")]
    fn mongo_table_def() -> TableDef<MongoColumnType>;

    #[cfg(feature = "mongo")]
    fn mysql_table_def() -> TableDef<MySQLColumnType>;

    #[cfg(feature = "postgres")]
    fn postgres_table_def() -> TableDef<PostgresColumnType>;

    #[cfg(feature = "sqlite")]
    fn sqlite_table_def() -> TableDef<SQLiteColumnType>;
}
