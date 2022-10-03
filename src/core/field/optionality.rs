use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub enum Optionality {
    Optional,
    Required,
    PresentWith(Vec<String>),
    PresentWithout(Vec<String>),
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
