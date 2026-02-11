use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum ColumnType {
    Bool,
    String,
    Date,
    ObjectId,
    Double,
    Int32,
    Long,
    Decimal128,
    Timestamp,
    UUID,
}

impl FromStr for ColumnType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "bool" => Self::Bool,
            "string" => Self::String,
            "date" => Self::Date,
            "objectid" => Self::ObjectId,
            "double" => Self::Double,
            "int32" => Self::Int32,
            "long" => Self::Long,
            "decimal128" => Self::Decimal128,
            "timestamp" => Self::Timestamp,
            "uuid" => Self::UUID,
            _ => Err(Error::new(s))?
        })
    }
}

impl ToString for ColumnType {

    fn to_string(&self) -> String {
        match self {
            ColumnType::Bool => "bool".to_string(),
            ColumnType::String => "string".to_string(),
            ColumnType::Date => "date".to_string(),
            ColumnType::ObjectId => "objectId".to_string(),
            ColumnType::Double => "double".to_string(),
            ColumnType::Int32 => "int32".to_string(),
            ColumnType::Long => "long".to_string(),
            ColumnType::Decimal128 => "decimal128".to_string(),
            ColumnType::Timestamp => "timestamp".to_string(),
            ColumnType::UUID => "uuid".to_string(),
        }
    }
}
