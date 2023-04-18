use crate::core::field::field::Field;

use crate::parser::ast::argument::Argument;

pub(crate) fn auto_increment_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.auto_increment = true;
}
