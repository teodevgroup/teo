use crate::core::field::Field;
use crate::parser::ast::argument::Argument;

pub(crate) fn id_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.primary = true;
}
