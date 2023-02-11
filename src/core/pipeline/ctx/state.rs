use crate::core::pipeline::ctx::validity::Validity;
use crate::prelude::{Error, Value};

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
}

impl Default for State {
    fn default() -> Self {
        State::Value(Value::Null)
    }
}
