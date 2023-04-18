use crate::core::database::r#type::DatabaseType;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};

#[derive(Debug, Clone, Copy)]
pub enum DatabaseName {
    MySQL,
    PostgreSQL,
    SQLite,
    MongoDB,
}

impl DatabaseName {
    pub(crate) fn default_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match self {
            DatabaseName::MongoDB => default_database_type_mongodb(field_type),
            DatabaseName::MySQL => default_database_type_mysql(field_type),
            DatabaseName::PostgreSQL => default_database_type_postgresql(field_type),
            DatabaseName::SQLite => default_database_type_sqlite(field_type),
        }
    }
}

fn default_database_type_mongodb(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::ObjectId => DatabaseType::ObjectId,
        FieldType::Bool => DatabaseType::Bool,
        FieldType::I32 => DatabaseType::Int32,
        FieldType::I64 => DatabaseType::Int64,
        FieldType::F32 => DatabaseType::Double { m: None, d: None },
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::Decimal => DatabaseType::Decimal { m: None, d: None },
        FieldType::String => DatabaseType::String,
        FieldType::Date => DatabaseType::DateTime(3),
        FieldType::DateTime => DatabaseType::DateTime(3),
        FieldType::Enum(_) => DatabaseType::String,
        FieldType::Vec(inner) => DatabaseType::Vec(Box::new(default_database_type_mongodb(inner.field_type()))),
        FieldType::HashMap(_) => panic!(""),
        FieldType::BTreeMap(_) => panic!(""),
        FieldType::Object(_) => panic!(""),
    }
}

fn default_database_type_mssql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        _ => panic!("Unhandled."),
    }
}

fn default_database_type_mysql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::TinyInt { m: Some(1), u: false },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::F32 => DatabaseType::Float { m: None, d: None },
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::VarChar { m: 191, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::DateTime(3),
        FieldType::Enum(enum_def) => DatabaseType::Enum(enum_def.into()),
        FieldType::Decimal => DatabaseType::Decimal { m: Some(65), d: Some(30) },
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}

fn default_database_type_postgresql(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::Bool,
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::Timestamp { p: 3, z: false },
        FieldType::Decimal => DatabaseType::Decimal { m: Some(65), d: Some(30) },
        FieldType::Enum(_) => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Vec(inner) => DatabaseType::Vec(Box::new(default_database_type_postgresql(inner.field_type()))),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}

fn default_database_type_sqlite(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::Int { m: None, u: false, },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::Int { m: None, u: false },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Real,
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::DateTime => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Decimal => DatabaseType::Decimal { m: None, d: None },
        FieldType::Enum(_) => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}
