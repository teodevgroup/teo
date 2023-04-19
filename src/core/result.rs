use crate::core::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait IntoTeoResult {
    fn io_result_into_teo_result(self) -> Result<T>;
}

impl<T> IntoTeoResult for std::io::Result<T> {
    fn io_result_into_teo_result(self) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }
}