use std::fmt::{Display, Result};

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(column_type: impl Into<String>) -> Self {
        Self(column_type.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.write_str("Unknown column type: ")?;
        f.write_str(&self.0)?;
        Ok(())
    }
}
