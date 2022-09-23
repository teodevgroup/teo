use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;

pub(crate) fn inferred_database_type_postgresql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        _ => DatabaseType::Undefined,
    }
}
