use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::db_type::DatabaseType;

impl ToSQLString for DatabaseType {
    fn to_string(&self, dialect: SQLDialect) -> String {
        match self {
            DatabaseType::Undefined => panic!("Unhandled undefined database type."),
            DatabaseType::ObjectId => panic!(),
            DatabaseType::Bool => if dialect == SQLDialect::MySQL {
                "TINYINT(1)".to_string()
            } else {
                "Bool".to_string()
            },
            DatabaseType::Bit { m } => format!("BIT({m})"),
            DatabaseType::BitVarying => "BIT VARYING".to_string(),
            DatabaseType::TinyInt { m , u } => (if *u { "TINYINT UNSIGNED" } else { "TINYINT" }).to_string(),
            DatabaseType::SmallInt { m, u } => (if *u { "SMALLINT UNSIGNED" } else { "SMALLINT" }).to_string(),
            DatabaseType::MediumInt { m, u } => (if *u { "MEDIUMINT UNSIGNED" } else { "MEDIUMINT" }).to_string(),
            DatabaseType::Int { m, u } => (if *u { "INT UNSIGNED" } else { "INT" }).to_string(),
            DatabaseType::BigInt { m, u } => (if *u { "BIGINT UNSIGNED" } else { "BIGINT" }).to_string(),
            DatabaseType::Decimal(_, _) => todo!(),
            DatabaseType::Float { m: _m, d: _d } => format!("FLOAT"),
            DatabaseType::Double => {
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
            DatabaseType::Timestamp(fsp, tz) => {
                if dialect == SQLDialect::PostgreSQL {
                    let tzinfo = if *tz { " WITH TIME ZONE" } else { "" };
                    format!("TIMESTAMP({fsp}){tzinfo}")
                } else {
                    format!("TIMESTAMP({fsp})")
                }
            }
            DatabaseType::Time(_, _) => todo!(),
            DatabaseType::Year => "YEAR".to_string(),
            DatabaseType::Char(l, cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("CHAR({l}){charset}{collation}")
            }
            DatabaseType::VarChar(l, cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("VARCHAR({l}){charset}{collation}")
            }
            DatabaseType::TinyText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("TINYTEXT{charset}{collation}")
            }
            DatabaseType::MediumText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("MEDIUMTEXT{charset}{collation}")
            }
            DatabaseType::LongText(cs, co) => {
                let charset = if let Some(v) = cs {
                    format!(" CHARACTER SET {v}")
                } else { "".to_string() };
                let collation = if let Some(v) = co {
                    format!(" COLLATION {v}")
                } else { "".to_string() };
                format!("LONGTEXT{charset}{collation}")
            }
            DatabaseType::Text(l, cs, co) => {
                if dialect == SQLDialect::PostgreSQL {
                    "TEXT".to_string()
                } else {
                    let charset = if let Some(v) = cs {
                        format!(" CHARACTER SET {v}")
                    } else { "".to_string() };
                    let collation = if let Some(v) = co {
                        format!(" COLLATION {v}")
                    } else { "".to_string() };
                    format!("TINYTEXT{charset}{collation}")
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
        }
    }
}
