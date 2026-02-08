use super::super::migration::{EnumDef, TableDef};

pub trait Schema {
    fn enum_defs() -> Vec<EnumDef>;
    fn table_defs() -> Vec<TableDef>;
}
