use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum ColumnType {
    BigInt,
    BigSerial,
    Bit { n: usize },
    BitVarying { n: Option<usize> },
    Boolean,
    Box,
    ByteA,
    Character { n: usize },
    CharacterVarying { n: Option<usize> },
    CIDR,
    Circle,
    Date,
    DoublePrecision,
    INet,
    Integer,
    JSON,
    JSONB,
    Line,
    LSeg,
    MACAddr,
    MACAddr8,
    Money,
    Numeric { p: Option<usize>, s: Option<usize> },
    Path,
    PgLSN,
    PGSnapshot,
    Point,
    Polygon,
    Real,
    SmallInt,
    SmallSerial,
    Serial,
    Text,
    TimeWithoutTimeZone { p: usize },
    TimeWithTimeZone { p: usize },
    TimestampWithoutTimeZone { p: usize },
    TimestampWithTimeZone { p: usize },
    TSQuery,
    TSVector,
    TxIDSnapshot,
    UUID,
    XML,
}

impl FromStr for ColumnType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "bigint" | "int8" => Self::BigInt,
            "bigserial" | "serial8" => Self::BigSerial,
            "bit varying" | "varbit" => Self::BitVarying { n: None },
            "boolean" | "bool" => Self::Boolean,
            "box" => Self::Box,
            "bytea" => Self::ByteA,
            "character" | "char" => Self::Character { n: 1 },
            "character varying" | "varchar" => Self::CharacterVarying { n: None },
            "cidr" => Self::CIDR,
            "circle" => Self::Circle,
            "date" => Self::Date,
            "double precision" | "float" | "float8" => Self::DoublePrecision,
            "inet" => Self::INet,
            "integer" | "int" | "int4" => Self::Integer,
            "json" => Self::JSON,
            "jsonb" => Self::JSONB,
            "line" => Self::Line,
            "lseg" => Self::LSeg,
            "macaddr" => Self::MACAddr,
            "macaddr8" => Self::MACAddr8,
            "money" => Self::Money,
            "numeric" | "decimal" => Self::Numeric { p: None, s: None },
            "path" => Self::Path,
            "pg_lsn" => Self::PgLSN,
            "pg_snapshot" => Self::PGSnapshot,
            "point" => Self::Point,
            "polygon" => Self::Polygon,
            "real" | "float4" => Self::Real,
            "smallint" | "int2" => Self::SmallInt,
            "smallserial" | "serial2" => Self::SmallSerial,
            "serial" | "serial4" => Self::Serial,
            "text" => Self::Text,
            "tsquery" => Self::TSQuery,
            "tsvector" => Self::TSVector,
            "txid_snapshot" => Self::TxIDSnapshot,
            "uuid" => Self::UUID,
            "xml" => Self::XML,
            "time with time zone" => Self::TimeWithTimeZone { p: 6 },
            "time without time zone" => Self::TimeWithoutTimeZone { p: 6 },
            "timetz" => Self::TimeWithTimeZone { p: 6 },
            "timestamp with time zone" => Self::TimestampWithTimeZone { p: 6 },
            "timestamp without time zone" => Self::TimestampWithoutTimeZone { p: 6 },
            "timestamptz" => Self::TimestampWithTimeZone { p: 6 },
            c if c.contains("(") && c.contains(")") => {
                let start = c.find("(").ok_or(Error::new(s))?;
                let end = c.find(")").ok_or(Error::new(s))?;
                let prefix = c[0..start].trim();
                let arg = c[start + 1..end].trim();
                let suffix = c[end + 1..].trim();
                match (prefix, suffix) {
                    ("bit", "") => {
                        let n = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Bit { n }
                    },
                    ("bit varying", "") | ("varbit", "") => {
                        let n = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::BitVarying { n: Some(n) }
                    },
                    ("character", "") | ("char", "") => {
                        let n = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Character { n }
                    },
                    ("character varying", "") | ("varchar", "") => {
                        let n = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::CharacterVarying { n: Some(n) }
                    },
                    ("numeric", "") | ("decimal", "") => {
                        let mut args = arg.split(",").into_iter().map(|a| a.trim());
                        let arg_p = if let Some(p) = args.next() {
                            usize::from_str(p).map_err(|_| Error::new(s))?
                        } else {
                            Err(Error::new(s))?
                        };
                        let arg_s = if let Some(s) = args.next() {
                            usize::from_str(s).map_err(|_| Error::new(s))?
                        } else {
                            Err(Error::new(s))?
                        };
                        if args.next() != None {
                            Err(Error::new(s))?
                        }
                        Self::Numeric { p: Some(arg_p), s: Some(arg_s) }
                    },
                    ("time", "without time zone") => {
                        let p = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::TimeWithoutTimeZone { p }
                    },
                    ("time", "with time zone") | ("timetz", "") => {
                        let p = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::TimeWithTimeZone { p }
                    },
                    ("timestamp", "without time zone") => {
                        let p = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::TimestampWithoutTimeZone { p }
                    },
                    ("timestamp", "with time zone") | ("timestamptz", "") => {
                        let p = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::TimestampWithTimeZone { p }
                    },
                    _ => Err(Error::new(s))?
                }
            }
            _ => Err(Error::new(s))?
        })
    }
}

impl ToString for ColumnType {
    fn to_string(&self) -> String {
        match self {
            ColumnType::BigInt => "bigint".to_string(),
            ColumnType::BigSerial => "bigserial".to_string(),
            ColumnType::Bit { n } => format!("bit({n})"),
            ColumnType::BitVarying { n } => if let Some(n) = n {
                format!("bit varying({n})")
            } else {
                "bit varying".to_string()
            },
            ColumnType::Boolean => "boolean".to_string(),
            ColumnType::Box => "box".to_string(),
            ColumnType::ByteA => "bytea".to_string(),
            ColumnType::Character { n } => format!("char({n})"),
            ColumnType::CharacterVarying { n } => if let Some(n) = n {
                format!("varchar({n})")
            } else {
                "varchar".to_string()
            },
            ColumnType::CIDR => "cidr".to_string(),
            ColumnType::Circle => "circle".to_string(),
            ColumnType::Date => "date".to_string(),
            ColumnType::DoublePrecision => "double precision".to_string(),
            ColumnType::INet => "inet".to_string(),
            ColumnType::Integer => "integer".to_string(),
            ColumnType::JSON => "json".to_string(),
            ColumnType::JSONB => "jsonb".to_string(),
            ColumnType::Line => "line".to_string(),
            ColumnType::LSeg => "lSeg".to_string(),
            ColumnType::MACAddr => "macaddr".to_string(),
            ColumnType::MACAddr8 => "macaddr8".to_string(),
            ColumnType::Money => "money".to_string(),
            ColumnType::Numeric { p, s } => if let Some(p) = p && let Some(s) = s {
                format!("numeric({p},{s})")
            } else if let Some(p) = p {
                format!("numeric({p})")
            } else {
                "numeric".to_string()
            },
            ColumnType::Path => "path".to_string(),
            ColumnType::PgLSN => "pg_lsn".to_string(),
            ColumnType::PGSnapshot => "pg_snapshot".to_string(),
            ColumnType::Point => "point".to_string(),
            ColumnType::Polygon => "polygon".to_string(),
            ColumnType::Real => "real".to_string(),
            ColumnType::SmallInt => "smallint".to_string(),
            ColumnType::SmallSerial => "smallserial".to_string(),
            ColumnType::Serial => "serial".to_string(),
            ColumnType::Text => "text".to_string(),
            ColumnType::TimeWithoutTimeZone { p } => if *p == 6 {
                "time without time zone".to_string()
            } else {
                format!("time({p}) without time zone")
            },
            ColumnType::TimeWithTimeZone { p } => if *p == 6 {
                "time with time zone".to_string()
            } else {
                format!("time({p}) with time zone")
            },
            ColumnType::TimestampWithoutTimeZone { p } => if *p == 6 {
                "timestamp without time zone".to_string()
            } else {
                format!("timestamp({p}) without time zone")
            },
            ColumnType::TimestampWithTimeZone { p } => if *p == 6 {
                "timestamp with time zone".to_string()
            } else {
                format!("timestamp({p}) with time zone")
            },
            ColumnType::TSQuery => "tsquery".to_string(),
            ColumnType::TSVector => "tsvector".to_string(),
            ColumnType::TxIDSnapshot => "txid_snapshot".to_string(),
            ColumnType::UUID => "uuid".to_string(),
            ColumnType::XML => "xml".to_string(),
        }
    }
}
