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
