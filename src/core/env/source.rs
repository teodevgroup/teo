use crate::prelude::{Object, Value};
use crate::core::env::source::Source::*;

#[derive(Clone)]
pub(crate) enum Source {
    DataBrowser,
    Identity(Option<Object>),
    CustomCode,
}

impl Source {

    pub(crate) fn is_data_browser(&self) -> bool {
        match self {
            DataBrowser => true,
            _ => false,
        }
    }

    pub(crate) fn is_identity(&self) -> bool {
        match self {
            Identity(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_custom_browser(&self) -> bool {
        match self {
            CustomCode => true,
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
            Identity(_) => Some(self.as_identity().into()),
            _ => None
        }
    }
}
