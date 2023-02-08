use crate::core::field::{Field, FieldIndex, IndexSettings};
use crate::parser::ast::argument::Argument;

pub(crate) fn unique_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.index = Some(FieldIndex::Unique(IndexSettings::default()));
}
