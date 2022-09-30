use key_path::KeyPath;
use crate::core::action::r#type::ActionType;
use crate::core::object::Object;
use crate::core::pipeline::context::Stage::{ConditionTrue, ConditionFalse, Default};
use crate::core::pipeline::context::Validity::{Invalid, Valid};
use crate::core::value::Value;

#[derive(Clone)]
pub enum Validity {
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

impl From<&str> for Validity {
    fn from(reason: &str) -> Self {
        Invalid(reason.to_string())
    }
}

impl From<String> for Validity {
    fn from(reason: String) -> Self {
        Invalid(reason)
    }
}

impl From<bool> for Validity {
    fn from(valid: bool) -> Self {
        match valid {
            true => Valid,
            false => Invalid("Value is invalid.".to_owned())
        }
    }
}

impl From<()> for Validity {
    fn from(_: ()) -> Self {
        Valid
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

#[derive(Copy, Clone, PartialEq)]
pub enum Intent {
    Create,
    Update,
    Delete,
    Authentication,
    ManyResult(ActionType),
    SingleResult(ActionType),
    NestedManyResult(ActionType),
    NestedSingleResult(ActionType),
    UserCodeGetProperty,
    UserCodeSetProperty,
}

#[derive(Clone)]
pub struct Context<'a> {
    pub(crate) value: Value,
    pub(crate) object: Object,
    pub(crate) key_path: KeyPath<'a>,
    pub(crate) identity: Option<Object>,
    pub(crate) validity: Validity,
    pub(crate) stage: Stage,
    pub(crate) intent: Intent,
}

impl<'a> Context<'a> {

    pub(crate) fn initial_state(object: Object, intent: Intent) -> Self {
        Context {
            value: Value::Object(object.clone()),
            object: object.clone(),
            key_path: KeyPath::default(),
            identity: object.get_identity(),
            validity: Valid,
            stage: Default,
            intent,
        }
    }

    pub(crate) fn alter_key_path(&self, key_path: KeyPath<'a>) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            key_path,
            identity: self.identity.clone(),
            validity: self.validity.clone(),
            stage: self.stage.clone(),
            intent: self.intent.clone(),
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
            intent: self.intent.clone(),
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
            intent: self.intent.clone(),
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
            stage,
            intent: self.intent.clone(),
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
