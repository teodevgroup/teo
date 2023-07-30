use crate::prelude::{Object, Req, Value};
use self::Initiator::*;

#[derive(Clone)]
pub enum Initiator {
    Identity(Option<Object>, Req),
    DataClient(Req),
    ProgramCode(Option<Req>),
}

impl Initiator {

    pub(crate) fn is_data_client(&self) -> bool {
        match self {
            DataClient(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_identity(&self) -> bool {
        match self {
            Identity(_, _) => true,
            _ => false,
        }
    }

    pub(crate) fn is_program_code(&self) -> bool {
        match self {
            ProgramCode(_) => true,
            _ => false,
        }
    }

    pub(crate) fn as_identity(&self) -> Option<&Object> {
        match self {
            Identity(identity, _) => identity.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn as_identity_value(&self) -> Option<Value> {
        match self {
            Identity(_, _) => Some(self.as_identity().cloned().into()),
            _ => None
        }
    }

    pub(crate) fn as_req(&self) -> Option<Req> {
        match self {
            Identity(_, req) => Some(req.clone()),
            DataClient(req) => Some(req.clone()),
            ProgramCode(req) => req.clone(),
        }
    }
}
