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
    let r#type: &str = r#type.to_lowercase().as_str();
    let regex = Regex::new("(.+)( (.+))?\\((.+)\\)").unwrap();
    match regex.captures(r#type) {
        None => panic!("Unhandled database type."),
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
            }
            if name == "int" {

            } else if name == "tinyint" {

            } else if name == "smallint" {

            }
        }
    }
    if r#type == "int" {
        DatabaseType::Int(false)
    } else if r#type == "int unsigned" {
        DatabaseType::Int(true)
    } else if r#type.starts_with("varchar") {

        let captures = regex.captures(r#type).unwrap();
        let num_str = captures.get(1).unwrap().as_str();
        let num = u16::from_str(num_str).unwrap();
        DatabaseType::VarChar(num, None, None)
    } else if r#type.starts_with("tinyint") {
        // tinyint(1)
    } else {
        panic!("Unhandled database type.")
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
