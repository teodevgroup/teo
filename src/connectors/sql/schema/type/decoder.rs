use std::str::FromStr;
use regex::Regex;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::db_type::DatabaseType;

pub(crate) struct SQLTypeDecoder { }

impl SQLTypeDecoder {
    pub(crate) fn decode(r#type: &str, dialect: SQLDialect) -> DatabaseType {
        match dialect {
            SQLDialect::MySQL => mysql_type_to_database_type(r#type),
            SQLDialect::PostgreSQL => postgresql_type_to_database_type(r#type),
            SQLDialect::SQLite => sqlite_type_to_database_type(r#type),
            SQLDialect::MSSQL => mssql_type_to_database_type(r#type),
        }
    }
}

fn mysql_type_to_database_type(r#type: &str) -> DatabaseType {
    let r#type_string = r#type.to_lowercase();
    let r#type: &str = r#type_string.as_str();
    let regex = Regex::new("(.+)( (.+))?\\((.+)\\)?").unwrap();
    match regex.captures(r#type) {
        None => panic!("Unhandled database type '{}' '{}'.", r#type, regex),
        Some(captures) => {
            let name = captures.get(1).unwrap().as_str();
            let trailing1 = captures.get(3).map(|m| m.as_str());
            let arg = captures.get(4).map(|m| m.as_str());
            match name {
                "bit" => DatabaseType::Bit { m: arg.map(|a| u8::from_str(a).unwrap()) },
                "tinyint" => DatabaseType::TinyInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "smallint" => DatabaseType::SmallInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "mediumint" => DatabaseType::MediumInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "int" => DatabaseType::Int { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "bigint" => DatabaseType::BigInt { m: arg.map(|a| u8::from_str(a).unwrap()), u: trailing1.is_some() },
                "float" => DatabaseType::Float { m: None, d: None },
                "double" => DatabaseType::Double { m: None, d: None },
                "char" => DatabaseType::Char { m: arg.map(|a| u8::from_str(a).unwrap()), n: None, c: None },
                "varchar" => DatabaseType::VarChar { m: arg.map(|a| u16::from_str(a).unwrap()).unwrap(), n: None, c: None },
                _ => panic!("Unhandled type.")
            }
        }
    }
}

fn postgresql_type_to_database_type(r#type: &str) -> DatabaseType {
    match r#type.to_lowercase().as_str() {
        _ => panic!("Unhandled database type.")
    }
}

fn sqlite_type_to_database_type(r#type: &str) -> DatabaseType {
    match r#type.to_lowercase().as_str() {
        _ => panic!("Unhandled database type.")
    }
}

fn mssql_type_to_database_type(r#type: &str) -> DatabaseType {
    match r#type.to_lowercase().as_str() {
        _ => panic!("Unhandled database type.")
    }
}
