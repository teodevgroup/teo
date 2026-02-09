#[cfg(feature = "mongo")]
use teo_column_type::MongoColumnType;
#[cfg(feature = "mysql")]
use teo_column_type::MySQLColumnType;
#[cfg(feature = "postgres")]
use teo_column_type::PostgresColumnType;
#[cfg(feature = "sqlite")]
use teo_column_type::SQLiteColumnType;

use super::super::migration::{EnumDef, TableDef};

pub trait Schema {

    #[cfg(any(feature = "mysql", feature = "postgres"))]
    fn enum_defs() -> Vec<EnumDef>;

    #[cfg(feature = "mongo")]
    fn mongo_table_defs() -> Vec<TableDef<MongoColumnType>>;

    #[cfg(feature = "mysql")]
    fn mysql_table_defs() -> Vec<TableDef<MySQLColumnType>>;

    #[cfg(feature = "postgres")]
    fn postgres_table_defs() -> Vec<TableDef<PostgresColumnType>>;

    #[cfg(feature = "sqlite")]
    fn sqlite_table_defs() -> Vec<TableDef<SQLiteColumnType>>;

}
