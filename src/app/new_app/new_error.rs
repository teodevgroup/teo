use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct FatalError(Cow<'static, str>);

#[derive(Debug)]
pub struct ServerError(Cow<'static, str>);

#[derive(Debug)]
pub enum RuntimeError {
    RuntimeError1,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RuntimeError::*;
        match self {
            RuntimeError1 => f.write_str("Runtime error 1")
        }
    }
}

#[derive(Debug)]
pub struct UserError {
    r#type: Cow<'static, str>,
    message: Cow<'static, str>,
    errors: Option<HashMap<Cow<'static, str>, Cow<'static, str>>>,
}

#[derive(Debug)]
pub enum Error {
    FatalError(FatalError),
    RuntimeError(RuntimeError),
    UserError(UserError),
    ServerError(ServerError),
}

impl Error {
    pub fn fatal(message: &'static str) -> Self {
        Self::FatalError(FatalError(Cow::Borrowed(message)))
    }

    pub fn fatal_message(message: String) -> Self {
        Self::FatalError(FatalError(Cow::Owned(message)))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            FatalError(err) => f.write_str(err.0.as_ref()),
            RuntimeError(err) => Display::fmt(err, f),
            UserError(err) => f.write_str(err.message.as_ref()),
            ServerError(err) => f.write_str(err.0.as_ref()),
        }
    }
}

impl std::error::Error for Error { }