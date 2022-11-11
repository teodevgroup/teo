use crate::core::field::Field;
use crate::core::field::read_rule::ReadRule;
use crate::core::field::write_rule::WriteRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn internal_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.read_rule = ReadRule::NoRead;
    field.write_rule = WriteRule::NoWrite;
}
