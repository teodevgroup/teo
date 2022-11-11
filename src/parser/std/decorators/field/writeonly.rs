use crate::core::field::Field;
use crate::core::field::read_rule::ReadRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn writeonly_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.read_rule = ReadRule::NoRead;
}
