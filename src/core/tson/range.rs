use crate::prelude::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub(crate) open: bool,
    pub(crate) start: Box<Value>,
    pub(crate) end: Box<Value>,
}
