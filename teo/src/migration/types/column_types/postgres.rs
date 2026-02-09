use std::str::FromStr;
use crate::migration::types::column_types::error::ColumnTypeError;

pub enum PostgresColumnType {
    BigInt,
}

impl FromStr for PostgresColumnType {
    type Err = ColumnTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase() {

        }
    }
}
