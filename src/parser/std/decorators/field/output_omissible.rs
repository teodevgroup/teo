use crate::core::field::field::Field;
use crate::parser::ast::argument::Argument;

pub(crate) fn output_omissible_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.output_omissible = true;
}
