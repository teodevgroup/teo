use crate::core::pipeline::Pipeline;
use crate::prelude::Value;

pub(crate) struct FieldMigration {
    renamed: Vec<String>,
    version: Option<String>,
    default: Option<Value>,
    action: Option<Pipeline>,
    priority: Option<usize>,
}
