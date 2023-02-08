use crate::prelude::{Object, Value};
use self::ActionSource::*;

#[derive(Clone)]
pub(crate) enum ActionSource {
    Identity(Option<Object>),
    DataClient,
    ProgramCode,
}

impl ActionSource {

    pub(crate) fn is_data_client(&self) -> bool {
        match self {
            DataClient => true,
            _ => false,
        }
    }

    pub(crate) fn is_identity(&self) -> bool {
        match self {
            Identity(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_program_code(&self) -> bool {
        match self {
            ProgramCode => true,
            _ => false,
        }
    }

    pub(crate) fn as_identity(&self) -> Option<&Object> {
        match self {
            Identity(identity) => identity.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn as_identity_value(&self) -> Option<Value> {
        match self {
            Identity(_) => Some(self.as_identity().cloned().into()),
            _ => None
        }
    }
}
