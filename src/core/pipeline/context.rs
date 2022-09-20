use crate::core::key_path::KeyPathItem;
use crate::core::object::Object;
use crate::core::pipeline::context::Stage::{ConditionTrue, ConditionFalse, Default};
use crate::core::pipeline::context::Validity::{Invalid, Valid};
use crate::core::value::Value;

#[derive(Clone)]
pub(crate) enum Validity {
    Valid,
    Invalid(String)
}

impl Validity {
    pub(crate) fn is_valid(&self) -> bool {
        match self {
            Valid => true,
            _ => false
        }
    }

    pub(crate) fn reason(&self) -> Option<&str> {
        match self {
            Invalid(reason) => Some(&reason),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub(crate) enum Stage {
    Default,
    ConditionTrue,
    ConditionFalse,
}

impl Stage {

    pub(crate) fn is_condition_default(&self) -> bool {
        match self {
            Default => true,
            _ => false
        }
    }

    pub(crate) fn is_condition_true(&self) -> bool {
        match self {
            ConditionTrue => true,
            _ => false
        }
    }

    pub(crate) fn is_condition_false(&self) -> bool {
        match self {
            ConditionFalse => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Purpose {
    Create,
    Update,
    Delete,
    Authentication,
}

#[derive(Clone)]
pub struct Context {
    pub(crate) value: Value,
    pub(crate) object: Object,
    pub(crate) key_path: Vec<KeyPathItem>,
    pub(crate) identity: Option<Object>,
    pub(crate) validity: Validity,
    pub(crate) stage: Stage,
    pub(crate) purpose: Purpose,
}

impl Context {

    pub(crate) fn initial_state(object: Object, purpose: Purpose) -> Self {
        Context {
            value: Value::Object(object.clone()),
            object: object.clone(),
            key_path: Vec::new(),
            identity: object.get_identity(),
            validity: Valid,
            stage: Default,
            purpose,
        }
    }

    pub(crate) fn alter_key_path(&self, key_path: Vec<KeyPathItem>) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path,
            identity: self.identity.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
            purpose: self.purpose.clone(),
        }
    }

    pub(crate) fn alter_value(&self, value: Value) -> Self {
        Self {
            value,
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            identity: self.identity.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
            purpose: self.purpose.clone(),
        }
    }

    pub(crate) fn alter_validity(&self, validity: Validity) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path: self.key_path.clone(),
            identity: self.identity.clone(),
            validity,
            stage: self.stage.clone(),
            purpose: self.purpose.clone(),
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
            identity: self.identity.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
            purpose: self.purpose.clone(),
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
}
