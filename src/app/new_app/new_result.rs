use super::new_error::Error;

pub type Result<T> = std::result::Result<T, Error>;
