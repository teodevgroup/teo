use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum MongoColumnType {
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

impl FromStr for MongoColumnType {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "date" => Self::Date,
            "objectid" => Self::ObjectId,
            "double" => Self::Double,
            "int32" => Self::Int32,
            "long" => Self::Long,
            "decimal128" => Self::Decimal128,
            "timestamp" => Self::Timestamp,
            _ => Err(Error::new(s))?
        })
    }
}
