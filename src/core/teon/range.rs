use crate::prelude::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub(crate) closed: bool,
    pub(crate) start: Box<Value>,
    pub(crate) end: Box<Value>,
}
