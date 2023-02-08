use crate::core::pipeline::Pipeline;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub enum Optionality {
    Optional,
    Required,
    PresentWith(Vec<Value>),
    PresentWithout(Vec<Value>),
    PresentIf(Pipeline),
}

impl Optionality {
    pub(crate) fn is_optional(&self) -> bool {
        match self {
            Optionality::Required => false,
            _ => true,
        }
    }

    pub(crate) fn is_required(&self) -> bool {
        match self {
            Optionality::Required => true,
            _ => false
        }
    }
}
