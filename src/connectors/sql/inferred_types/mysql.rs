use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;

pub(crate) fn inferred_database_type_mysql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Undefined => DatabaseType::Undefined,
        FieldType::ObjectId => DatabaseType::Undefined,
        FieldType::Bool => DatabaseType::Bool,
        FieldType::I8 => DatabaseType::TinyInt(false),
        FieldType::I16 => DatabaseType::SmallInt(false),
        FieldType::I32 => DatabaseType::Int(false),
        FieldType::I64 => DatabaseType::BigInt(false),
        FieldType::I128 => DatabaseType::BigInt(false),
        FieldType::U8 => DatabaseType::TinyInt(true),
        FieldType::U16 => DatabaseType::SmallInt(true),
        FieldType::U32 => DatabaseType::Int(true),
        FieldType::U64 => DatabaseType::BigInt(true),
        FieldType::U128 => DatabaseType::BigInt(true),
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Double,
        FieldType::String => DatabaseType::VarChar(191, None, None),
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::DateTime(3),
        FieldType::Enum(_) => DatabaseType::Undefined,
        FieldType::Vec(_) => DatabaseType::Undefined,
        FieldType::Map(_) => DatabaseType::Undefined,
        FieldType::Object(_) => DatabaseType::Undefined,
        _ => DatabaseType::Undefined,
    }
}
