#[cfg(feature = "mongo")]
use teo_column_type::mongo;
#[cfg(feature = "mysql")]
use teo_column_type::mysql;
#[cfg(feature = "postgres")]
use teo_column_type::postgres;
#[cfg(feature = "sqlite")]
use teo_column_type::sqlite;

use super::super::migration::{EnumDef, TableDef};

pub trait Schema: Send {

    #[cfg(any(feature = "mysql", feature = "postgres"))]
    fn enum_defs() -> Vec<EnumDef>;

    #[cfg(feature = "mongo")]
    fn mongo_table_defs() -> Vec<TableDef<mongo::ColumnType>>;

    #[cfg(feature = "mysql")]
    fn mysql_table_defs() -> Vec<TableDef<mysql::ColumnType>>;

    #[cfg(feature = "postgres")]
    fn postgres_table_defs() -> Vec<TableDef<postgres::ColumnType>>;

    #[cfg(feature = "sqlite")]
    fn sqlite_table_defs() -> Vec<TableDef<sqlite::ColumnType>>;

}
