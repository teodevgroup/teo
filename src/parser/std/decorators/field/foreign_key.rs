use crate::core::field::Field;

use crate::parser::ast::argument::Argument;

pub(crate) fn foreign_key_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.foreign_key = true;
}
