use super::super::migration::{EnumDef, TableDef};

pub trait Schema {
    fn enum_defs() -> Vec<EnumDef>;
    fn table_defs() -> Vec<TableDef>;

    #[cfg(feature = "postgres")]
    fn postgres_table_defs() -> Vec<TableDef>;
}
