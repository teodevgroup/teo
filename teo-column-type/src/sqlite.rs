use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum ColumnType {
    Integer,
    Real,
    Text,
    Blob,
}

impl FromStr for ColumnType {

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

impl ToString for ColumnType {
    fn to_string(&self) -> String {
        match self {
            ColumnType::Integer => "integer".to_string(),
            ColumnType::Real => "real".to_string(),
            ColumnType::Text => "text".to_string(),
            ColumnType::Blob => "blob".to_string(),
        }
    }
}
