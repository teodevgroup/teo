use crate::core::field::builder::FieldBuilder;

use crate::parser::ast::argument::Argument;

pub(crate) fn auto_increment_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.auto_increment();
}
