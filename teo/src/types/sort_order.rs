use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum SortOrder {
    Asc,
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
