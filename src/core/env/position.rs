use crate::prelude::Object;
use crate::core::env::position::Position::*;

#[derive(Copy, Clone)]
pub(crate) enum Position {
    RootSingle,
    RootMany,
    NestedSingle,
    NestedMany,
}

impl Position {

    pub(crate) fn is_single(&self) -> bool {
        match self {
            RootSingle | NestedSingle => true,
            _ => false,
        }
    }

    pub(crate) fn is_many(&self) -> bool {
        match self {
            RootMany | NestedMany => true,
            _ => false,
        }
    }

    pub(crate) fn is_root(&self) -> bool {
        match self {
            RootSingle | RootMany => true,
            _ => false,
        }
    }

    pub(crate) fn is_nested(&self) -> bool {
        match self {
            NestedSingle | NestedMany => true,
            _ => false,
        }
    }
}
