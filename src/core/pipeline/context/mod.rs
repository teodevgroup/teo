use key_path::KeyPath;

use crate::core::object::Object;
use crate::core::pipeline::context::validity::Validity;
use crate::core::pipeline::context::validity::Validity::{Invalid, Valid};
use crate::core::tson::Value;

pub mod validity;

#[derive(Clone)]
pub struct Context<'a> {
    pub(crate) value: Value,
    pub(crate) object: Option<Object>,
    pub(crate) key_path: KeyPath<'a>,
    pub(crate) validity: Validity,
}

impl<'a> Context<'a> {

    pub(crate) fn initial_state_with_value(value: Value) -> Self {
        Context {
            value,
            object: None,
            key_path: KeyPath::default(),
            validity: Valid,
        }
    }

    pub(crate) fn initial_state_with_object(object: Object) -> Self {
        Context {
            value: Value::Object(object.clone()),
            object: Some(object.clone()),
            key_path: KeyPath::default(),
            validity: Valid,
        }
    }

    pub(crate) fn alter_key_path(&self, key_path: KeyPath<'a>) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path,
            validity: self.validity.clone(),
        }
    }

    pub(crate) fn alter_value_with_identity(&self) -> Self {
        Self {
            value: self.object.as_ref().unwrap().env().source().as_identity_value().or(Some(Value::Null)).unwrap(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity: self.validity.clone(),
        }
    }

    pub(crate) fn alter_value(&self, value: Value) -> Self {
        Self {
            value,
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity: self.validity.clone(),
        }
    }

    pub(crate) fn alter_validity(&self, validity: Validity) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity,
        }
    }

    pub(crate) fn invalid(&self, reason: impl Into<String>) -> Self {
        self.alter_validity(Invalid(reason.into()))
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.validity.is_valid()
    }

    pub(crate) fn invalid_reason(&self) -> Option<&str> {
        self.validity.reason()
    }

    pub(crate) fn value(&self) -> &Value {
        &self.value
    }
}
