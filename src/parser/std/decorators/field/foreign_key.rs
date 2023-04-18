use crate::core::field::field::Field;

use crate::parser::ast::argument::Argument;

pub(crate) fn foreign_key_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.foreign_key = true;
    field.input_omissible = true;
}
