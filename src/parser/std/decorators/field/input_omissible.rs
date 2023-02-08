use crate::core::field::Field;
use crate::parser::ast::argument::Argument;

pub(crate) fn input_omissible_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.input_omissible = true;
}
