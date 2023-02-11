use key_path::KeyPath;
use crate::prelude::{Error, Value, Result};

#[derive(Clone)]
pub(crate) enum State {
    Value(Value),
    Invalid(String),
    Err(Error),
}

impl State {

    pub(crate) fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    pub(crate) fn as_value(&self) -> Option<&Value> {
        match self {
            State::Value(v) => Some(v),
            _ => None,
        }
    }

    pub(crate) fn is_invalid(&self) -> bool {
        self.as_invalid().is_some()
    }

    pub(crate) fn as_invalid(&self) -> Option<&str> {
        match self {
            State::Invalid(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn is_err(&self) -> bool {
        self.as_err().is_some()
    }

    pub(crate) fn as_err(&self) -> Option<&Error> {
        match self {
            State::Err(e) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn get_value<'a>(&self, path: impl AsRef<KeyPath<'a>>) -> Result<Value> {
        match self {
            State::Value(value) => Ok(value.clone()),
            State::Invalid(reason) => Err(Error::validation_error(path, reason)),
            State::Err(error) => Err(error.clone()),
        }
    }

    pub(crate) fn get_value_internal(&self) -> Result<Value> {
        match self {
            State::Value(value) => Ok(value.clone()),
            State::Invalid(reason) => Err(Error::internal_server_error(reason)),
            State::Err(error) => Err(error.clone()),
        }
    }

    pub(crate) fn permission<'a>(&self, path: impl AsRef<KeyPath<'a>>) -> Result<()> {
        match self {
            State::Value(_) => Ok(()),
            State::Invalid(reason) => Err(Error::permission_error(path, reason)),
            State::Err(error) => Err(error.clone()),
        }
    }

    pub(crate) fn is_valid(&self) -> Result<bool> {
        match self {
            State::Value(_) => Ok(true),
            State::Invalid(_) => Ok(false),
            State::Err(error) => Err(error.clone()),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Value(Value::Null)
    }
}
