pub mod state;
pub mod validity;

use key_path::KeyPath;
use crate::core::object::Object;
use crate::core::pipeline::ctx::state::State;
use crate::core::pipeline::ctx::validity::Validity;
use crate::core::pipeline::ctx::validity::Validity::{Invalid, Valid};
use crate::core::teon::Value;
use crate::prelude::Error;

#[derive(Clone)]
pub struct Ctx<'a> {
    pub(crate) state: State,
    pub(crate) object: Option<Object>,
    pub(crate) path: KeyPath<'a>,
}

impl<'a> Ctx<'a> {

    pub(crate) fn initial_state_with_value(value: Value) -> Self {
        Self {
            state: State::Value(value),
            object: None,
            path: KeyPath::default(),
        }
    }

    pub(crate) fn initial_state_with_object(object: Object) -> Self {
        Self {
            state: State::Value(Value::Null),
            object: Some(object),
            path: KeyPath::default(),
        }
    }

    pub(crate) fn with_path(&self, path: KeyPath<'a>) -> Self {
        Self {
            state: self.state.clone(),
            object: self.object.clone(),
            path,
        }
    }

    pub(crate) fn with_value(&self, value: Value) -> Self {
        Self {
            state: State::Value(value),
            object: self.object.clone(),
            path: self.path.clone(),
        }
    }

    pub(crate) fn with_invalid(&self, reason: impl Into<String>) -> Self {
        Self {
            state: State::Invalid(reason.into()),
            object: self.object.clone(),
            path: self.path.clone(),
        }
    }
}
