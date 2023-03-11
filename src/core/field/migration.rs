use crate::core::pipeline::Pipeline;
use crate::prelude::Value;

#[derive(Clone, Debug)]
pub(crate) struct FieldMigration {
    pub(crate) renamed: Vec<String>,
    pub(crate) version: Option<String>,
    pub(crate) default: Option<Value>,
    pub(crate) action: Option<Pipeline>,
    pub(crate) priority: Option<usize>,
    pub(crate) drop: bool,
}
