use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum ColumnType {
    TinyInt,
    SmallInt,
    MediumInt,
    Int,
    BigInt,
    Decimal { m: usize, d: usize },
    Float,
    Double,
    Bit { m: usize },
    Date,
    Time { fsp: usize },
    DateTime { fsp: usize },
    Timestamp { fsp: usize },
    Year,
    Char { m: usize },
    VarChar { m: usize },
    Binary { m: usize },
    VarBinary { m: usize },
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
    TinyText,
    Text,
    MediumText,
    LongText,
    Geometry,
    Point,
    LineString,
    Polygon,
    Multipoint,
    MultilineString,
    Multipolygon,
    GeometryCollection,
    JSON,
}

impl FromStr for ColumnType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "tinyint" => Self::TinyInt,
            "smallint" => Self::SmallInt,
            "mediumint" => Self::MediumInt,
            "int" | "integer" => Self::Int,
            "bigint" => Self::BigInt,
            "decimal" | "numeric" | "dec" | "fixed" => Self::Decimal { m: 10, d: 0 },
            "double" | "double precision" | "real" => Self::Double,
            "float" => Self::Float,
            "bit" => Self::Bit { m: 1 },
            "date" => Self::Date,
            "time" => Self::Time { fsp: 0 },
            "datetime" => Self::DateTime { fsp: 0 },
            "timestamp" => Self::Timestamp { fsp: 0 },
            "year" => Self::Year,
            "char" => Self::Char { m: 1 },
            "tinyblob" => Self::TinyBlob,
            "blob" => Self::Blob,
            "mediumblob" => Self::MediumBlob,
            "longblob" => Self::LongBlob,
            "tinytext" => Self::TinyText,
            "text" => Self::Text,
            "mediumtext" => Self::MediumText,
            "longtext" => Self::LongText,
            "geometry" => Self::Geometry,
            "point" => Self::Point,
            "linestring" => Self::LineString,
            "polygon" => Self::Polygon,
            "multipoint" => Self::Multipoint,
            "multilinestring" => Self::MultilineString,
            "multipolygon" => Self::Multipolygon,
            "geometrycollection" => Self::GeometryCollection,
            "json" => Self::JSON,
            c if c.contains("(") && c.contains(")") => {
                let start = c.find("(").ok_or(Error::new(s))?;
                let end = c.find(")").ok_or(Error::new(s))?;
                let prefix = c[0..start].trim();
                let arg = c[start + 1..end].trim();
                let suffix = c[end + 1..].trim();
                match (prefix, suffix) {
                    ("decimal", "") | ("numeric", "") | ("dec", "") | ("fixed", "") => {
                        let mut args = arg.split(",").into_iter().map(|a| a.trim());
                        let m = if let Some(m) = args.next() {
                            usize::from_str(m).map_err(|_| Error::new(s))?
                        } else {
                            Err(Error::new(s))?
                        };
                        if let Some(d) = args.next() {
                            if args.next() != None {
                                Err(Error::new(s))?
                            } else {
                                let d = usize::from_str(d).map_err(|_| Error::new(s))?;
                                Self::Decimal { m, d }
                            }
                        } else {
                            Self::Decimal { m, d: 0 }
                        }
                    },
                    ("float", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        match m {
                            0..=23 => Self::Float,
                            24..=53 => Self::Double,
                            _ => Err(Error::new(s))?
                        }
                    },
                    ("bit", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Bit { m }
                    },
                    ("time", "") => {
                        let fsp = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Time { fsp }
                    },
                    ("datetime", "") => {
                        let fsp = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::DateTime { fsp }
                    },
                    ("timestamp", "") => {
                        let fsp = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Timestamp { fsp }
                    },
                    ("char", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Char { m }
                    },
                    ("varchar", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::VarChar { m }
                    }
                    ("binary", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::Binary { m }
                    },
                    ("varbinary", "") => {
                        let m = usize::from_str(arg).map_err(|_| Error::new(s))?;
                        Self::VarBinary { m }
                    }
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
            ColumnType::TinyInt => "tinyint".to_string(),
            ColumnType::SmallInt => "smallint".to_string(),
            ColumnType::MediumInt => "mediumint".to_string(),
            ColumnType::Int => "int".to_string(),
            ColumnType::BigInt => "bigint".to_string(),
            ColumnType::Decimal { m, d } => format!("decimal({m},{d})"),
            ColumnType::Float => "float".to_string(),
            ColumnType::Double => "double".to_string(),
            ColumnType::Bit { m } => format!("bit({m})"),
            ColumnType::Date => "date".to_string(),
            ColumnType::Time { fsp } => format!("time({fsp})"),
            ColumnType::DateTime { fsp } => format!("datetime({fsp})"),
            ColumnType::Timestamp { fsp } => format!("timestamp({fsp})"),
            ColumnType::Year => "year".to_string(),
            ColumnType::Char { m } => format!("char({m})"),
            ColumnType::VarChar { m } => format!("varchar({m})"),
            ColumnType::Binary { m } => format!("binary({m})"),
            ColumnType::VarBinary { m } => format!("varbinary({m})"),
            ColumnType::TinyBlob => "tinyblob".to_string(),
            ColumnType::Blob => "blob".to_string(),
            ColumnType::MediumBlob => "mediumblob".to_string(),
            ColumnType::LongBlob => "longblob".to_string(),
            ColumnType::TinyText => "tinytext".to_string(),
            ColumnType::Text => "text".to_string(),
            ColumnType::MediumText => "mediumtext".to_string(),
            ColumnType::LongText => "longtext".to_string(),
            ColumnType::Geometry => "geometry".to_string(),
            ColumnType::Point => "point".to_string(),
            ColumnType::LineString => "linestring".to_string(),
            ColumnType::Polygon => "polygon".to_string(),
            ColumnType::Multipoint => "multipoint".to_string(),
            ColumnType::MultilineString => "multilinestring".to_string(),
            ColumnType::Multipolygon => "multipolygon".to_string(),
            ColumnType::GeometryCollection => "geometrycollection".to_string(),
            ColumnType::JSON => "json".to_string(),
        }
    }
}
