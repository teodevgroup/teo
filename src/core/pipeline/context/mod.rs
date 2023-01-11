use key_path::KeyPath;

use crate::core::object::Object;
use crate::core::pipeline::context::stage::Stage;
use crate::core::pipeline::context::stage::Stage::Default;
use crate::core::pipeline::context::validity::Validity;
use crate::core::pipeline::context::validity::Validity::{Invalid, Valid};
use crate::core::tson::Value;

pub mod validity;
pub(crate) mod stage;

#[derive(Clone)]
pub struct Context<'a> {
    pub(crate) value: Value,
    pub(crate) object: Option<Object>,
    pub(crate) key_path: KeyPath<'a>,
    pub(crate) validity: Validity,
    pub(crate) stage: Stage,
}

impl<'a> Context<'a> {

    pub(crate) fn initial_state_with_value(value: Value) -> Self {
        Context {
            value,
            object: None,
            key_path: KeyPath::default(),
            validity: Valid,
            stage: Default,
        }
    }

    pub(crate) fn initial_state_with_object(object: Object) -> Self {
        Context {
            value: Value::Object(object.clone()),
            object: Some(object.clone()),
            key_path: KeyPath::default(),
            validity: Valid,
            stage: Default,
        }
    }

    pub(crate) fn alter_key_path(&self, key_path: KeyPath<'a>) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path,
            validity: self.validity.clone(),
            stage: self.stage.clone(),
        }
    }

    pub(crate) fn alter_value_with_identity(&self) -> Self {
        Self {
            value: self.object.as_ref().unwrap().env().source().as_identity_value().or(Some(Value::Null)).unwrap(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
        }
    }

    pub(crate) fn alter_value(&self, value: Value) -> Self {
        Self {
            value,
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
        }
    }

    pub(crate) fn alter_validity(&self, validity: Validity) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity,
            stage: self.stage.clone(),
        }
    }

    pub(crate) fn invalid(&self, reason: impl Into<String>) -> Self {
        self.alter_validity(Invalid(reason.into()))
    }

    pub(crate) fn alter_stage(&self, stage: Stage) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            validity: self.validity.clone(),
            stage,
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.validity.is_valid()
    }

    pub(crate) fn invalid_reason(&self) -> Option<&str> {
        self.validity.reason()
    }

    pub(crate) fn is_condition_true(&self) -> bool {
        self.stage.is_condition_true()
    }

    pub(crate) fn is_condition_false(&self) -> bool {
        self.stage.is_condition_false()
    }

    pub(crate) fn value(&self) -> &Value {
        &self.value
    }
}
