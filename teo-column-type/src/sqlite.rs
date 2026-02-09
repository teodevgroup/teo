use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum SQLiteColumnType {
    Integer,
    Real,
    Text,
    Blob,
}

impl FromStr for SQLiteColumnType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            s if s.contains("int") => Self::Integer,
            "text" => Self::Text,
            s if s.contains("char") || s.contains("clob") => Self::Text,
            "real" => Self::Real,
            s if s.contains("floa") || s.contains("doub") => Self::Real,
            "blob" => Self::Blob,
            _ => Err(Error::new(s))?
        })
    }
}
