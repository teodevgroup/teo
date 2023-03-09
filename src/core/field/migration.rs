use crate::prelude::Value;

pub(crate) struct FieldMigration {
    default: Option<Value>,
    renamed: Vec<String>,
}
