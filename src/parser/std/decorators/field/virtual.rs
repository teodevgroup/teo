use crate::core::field::Field;


use crate::parser::ast::argument::Argument;

pub(crate) fn virtual_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.r#virtual = true;
}
