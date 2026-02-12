use std::{convert::Infallible, str::FromStr};
#[cfg(any(feature = "mongodb", feature = "serde"))]
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "mongodb", derive(Serialize, Deserialize))]
pub enum SortOrder {
    #[cfg_attr(any(feature = "mongodb", feature = "serde"), serde(rename = "asc"))]
    Asc,
    #[cfg_attr(any(feature = "mongodb", feature = "serde"), serde(rename = "desc"))]
    Desc,
}

impl FromStr for SortOrder {

    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "desc" => Ok(SortOrder::Desc),
            _ => Ok(SortOrder::Asc)
        }
    }
}

impl AsRef<str> for SortOrder {

    fn as_ref(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

impl SortOrder {

    #[cfg(feature = "mongo")]
    pub(crate) fn as_i32(&self) -> i32 {
        match self {
            SortOrder::Asc => 1,
            SortOrder::Desc => -1,
        }
    }
}
