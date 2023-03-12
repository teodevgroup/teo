use std::borrow::Cow;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::core::database::r#type::DatabaseType;

impl ToSQLString for DatabaseType {
    fn to_string(&self, dialect: SQLDialect) -> String {
        match self {
            DatabaseType::ObjectId => panic!(),
            DatabaseType::Bool => if dialect == SQLDialect::MySQL {
                "TINYINT(1)".to_string()
            } else if dialect == SQLDialect::PostgreSQL {
                "boolean".to_string()
            } else if dialect == SQLDialect::SQLite {
                "INTEGER".to_string()
            } else if dialect == SQLDialect::MSSQL {
                "bit".to_string()
            } else {
                "BOOL".to_string()
            },
            DatabaseType::Bit { m } => {
                let arg = if let Some(m) = m {
                    Cow::Owned(format!("({m})"))
                } else {
                    Cow::Borrowed("")
                };
                format!("BIT{arg}")
            },
            DatabaseType::BitVarying => "BIT VARYING".to_string(),
            DatabaseType::TinyInt { m: _ , u } => (if *u { "TINYINT UNSIGNED" } else { "TINYINT" }).to_string(),
            DatabaseType::SmallInt { m: _, u } => (if *u { "SMALLINT UNSIGNED" } else { "SMALLINT" }).to_string(),
            DatabaseType::MediumInt { m: _, u } => (if *u { "MEDIUMINT UNSIGNED" } else { "MEDIUMINT" }).to_string(),
            DatabaseType::Int { m: _, u } => if dialect == SQLDialect::MySQL {
                (if *u { "INT UNSIGNED" } else { "INT" }).to_string()
            } else if dialect == SQLDialect::PostgreSQL || dialect == SQLDialect::SQLite {
                "integer".to_string()
            } else {
                "INT".to_string()
            },
            DatabaseType::BigInt { m: _, u } => (if *u { "BIGINT UNSIGNED" } else { "BIGINT" }).to_string(),
            DatabaseType::Decimal { m, d } => if dialect == SQLDialect::PostgreSQL {
                let m = m.unwrap_or(65);
                let d = d.unwrap_or(30);
                format!("DECIMAL({}, {})", m, d)
            } else if dialect == SQLDialect::MySQL {
                let m = m.unwrap_or(65);
                let d = d.unwrap_or(30);
                format!("DECIMAL({}, {})", m, d)
            } else if dialect == SQLDialect::MSSQL {
                let m = m.unwrap_or(32);
                let d = d.unwrap_or(16);
                format!("DECIMAL({}, {})", m, d)
            } else {
                "DECIMAL".to_string()
            },
            DatabaseType::Float { m: _m, d: _d } => format!("FLOAT"),
            DatabaseType::Double { m: _m, d: _d } => {
                if dialect == SQLDialect::PostgreSQL {
                    "DOUBLE PRECISION".to_string()
                } else {
                    "DOUBLE".to_string()
                }
            }
            DatabaseType::Real => {
                if dialect == SQLDialect::MySQL {
                    "FLOAT".to_string()
                } else {
                    "REAL".to_string()
                }
            }
            DatabaseType::Date => "DATE".to_string(),
            DatabaseType::DateTime(fsp) => format!("DATETIME({fsp})"),
            DatabaseType::Timestamp { p, z } => {
                if dialect == SQLDialect::PostgreSQL {
                    let tzinfo = if *z { " WITH TIME ZONE" } else { "" };
                    format!("TIMESTAMP({p}){tzinfo}")
                } else {
                    format!("TIMESTAMP({p})")
                }
            }
            DatabaseType::Time(_, _) => todo!(),
            DatabaseType::Year => "YEAR".to_string(),
            DatabaseType::Char { m, n, c } => {
                let arg = if let Some(m) = m {
                    Cow::Owned(format!("({})", m))
                } else {
                    Cow::Borrowed("")
                };
                let charset = if let Some(v) = n {
                    Cow::Owned(format!(" CHARACTER SET {v}"))
                } else { Cow::Borrowed("") };
                let collation = if let Some(v) = c {
                    Cow::Owned(format!(" COLLATION {v}"))
                } else { Cow::Borrowed("") };
                format!("CHAR{arg}{charset}{collation}")
            }
            DatabaseType::VarChar { m, n, c } => {
                let arg = format!("({})", m);
                let charset = if let Some(v) = n {
                    Cow::Owned(format!(" CHARACTER SET {v}"))
                } else { Cow::Borrowed("") };
                let collation = if let Some(v) = c {
                    Cow::Owned(format!(" COLLATION {v}"))
                } else { Cow::Borrowed("") };
                format!("VARCHAR{arg}{charset}{collation}")
            }
            DatabaseType::TinyText { n , c } => {
                let charset = if let Some(v) = n {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = c {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("TINYTEXT{charset}{collation}")
            }
            DatabaseType::MediumText { n, c } => {
                let charset = if let Some(v) = n {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = c {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("MEDIUMTEXT{charset}{collation}")
            }
            DatabaseType::LongText { n, c } => {
                let charset = if let Some(v) = n {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = c {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("LONGTEXT{charset}{collation}")
            }
            DatabaseType::Text { m: _, n ,c } => {
                if dialect == SQLDialect::MySQL {
                    let charset = if let Some(v) = n {
                        format!(" CHARACTER SET {v}")
                    } else { "".to_string() };
                    let collation = if let Some(v) = c {
                        format!(" COLLATION {v}")
                    } else { "".to_string() };
                    format!("TEXT{charset}{collation}")
                } else {
                    "TEXT".to_string()
                }
            }
            DatabaseType::Binary(l) => format!("BINARY({l})"),
            DatabaseType::VarBinary(l) => format!("VARBINARY({l})"),
            DatabaseType::TinyBlob => "TINYBLOB".to_string(),
            DatabaseType::MediumBlob => "MEDIUMBLOB".to_string(),
            DatabaseType::LongBlob => "LONGBLOB".to_string(),
            DatabaseType::Blob(l) => format!("BLOB({l})"),
            DatabaseType::ByteA => "bytea".to_string(),
            DatabaseType::Int32 => panic!("SQL databases don't support Int32."),
            DatabaseType::Int64 => panic!("SQL databases don't support Int64."),
            DatabaseType::String => panic!("SQL databases don't support String."),
            DatabaseType::Vec(inner) => if dialect == SQLDialect::PostgreSQL {
                inner.to_string(dialect) + "[]"
            } else {
                panic!("Array is only supported for PostgreSQL.")
            }
        }
    }
}
