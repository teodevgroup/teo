use super::super::migration::TableDef;

pub trait Entity {
    fn table_def() -> TableDef;
}
