use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::r#type::FieldType;

pub trait ToDatabaseType {
    fn to_database_type(&self, dialect: SQLDialect) -> DatabaseType;
}

impl ToDatabaseType for FieldType {
    fn to_database_type(&self, dialect: SQLDialect) -> DatabaseType {
        match dialect {
            SQLDialect::SQLite => default_database_type_sqlite(self),
            SQLDialect::MySQL => default_database_type_mysql(self),
            SQLDialect::PostgreSQL => default_database_type_postgresql(self),
            SQLDialect::MSSQL => default_database_type_mssql(self),
        }
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
        FieldType::I8 => DatabaseType::TinyInt { m: None, u: false },
        FieldType::I16 => DatabaseType::SmallInt { m: None, u: false },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::I128 => DatabaseType::BigInt { m: None, u: false },
        FieldType::U8 => DatabaseType::TinyInt { m: None, u: true },
        FieldType::U16 => DatabaseType::SmallInt { m: None, u: true },
        FieldType::U32 => DatabaseType::Int { m: None, u: true },
        FieldType::U64 => DatabaseType::BigInt { m: None, u: true },
        FieldType::U128 => DatabaseType::BigInt { m: None, u: true },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::VarChar { m: 191, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::DateTime(3),
        FieldType::Enum(_) => DatabaseType::String,
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
        FieldType::I8 => DatabaseType::SmallInt { m: None, u: false },
        FieldType::I16 => DatabaseType::SmallInt { m: None, u: false },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::BigInt { m: None, u: false },
        FieldType::I128 => DatabaseType::BigInt { m: None, u: false },
        FieldType::U8 => DatabaseType::SmallInt { m: None, u: true },
        FieldType::U16 => DatabaseType::Int { m: None, u: true },
        FieldType::U32 => DatabaseType::BigInt { m: None, u: true },
        FieldType::U64 => DatabaseType::BigInt { m: None, u: true },
        FieldType::U128 => DatabaseType::BigInt { m: None, u: true },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Double { m: None, d: None },
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Date,
        FieldType::DateTime => DatabaseType::Timestamp { p: 3, z: false },
        FieldType::Enum(_) => panic!(),
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}

fn default_database_type_sqlite(field_type: &FieldType) -> DatabaseType {
    match field_type {
        FieldType::Bool => DatabaseType::Int { m: None, u: false, },
        FieldType::I8 => DatabaseType::Int { m: None, u: false },
        FieldType::I16 => DatabaseType::Int { m: None, u: false },
        FieldType::I32 => DatabaseType::Int { m: None, u: false },
        FieldType::I64 => DatabaseType::Int { m: None, u: false },
        FieldType::I128 => DatabaseType::Int { m: None, u: false },
        FieldType::U8 => DatabaseType::Int { m: None, u: true },
        FieldType::U16 => DatabaseType::Int { m: None, u: true },
        FieldType::U32 => DatabaseType::Int { m: None, u: true },
        FieldType::U64 => DatabaseType::Int { m: None, u: true },
        FieldType::U128 => DatabaseType::Int { m: None, u: true },
        FieldType::F32 => DatabaseType::Real,
        FieldType::F64 => DatabaseType::Real,
        FieldType::String => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Date => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::DateTime => DatabaseType::Text { m: None, n: None, c: None },
        FieldType::Enum(_) => panic!(),
        FieldType::Vec(_) => panic!(),
        FieldType::HashMap(_) => panic!(),
        FieldType::BTreeMap(_) => panic!(),
        FieldType::Object(_) => panic!(),
        _ => panic!(),
    }
}
