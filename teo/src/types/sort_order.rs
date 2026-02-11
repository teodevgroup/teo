#[derive(Debug, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl AsRef<str> for SortOrder {

    fn as_ref(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}
