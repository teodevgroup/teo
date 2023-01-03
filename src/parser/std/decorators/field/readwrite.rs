use crate::core::field::builder::FieldBuilder;
use crate::core::field::Field;
use crate::core::field::read_rule::ReadRule;
use crate::core::field::write_rule::WriteRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn readwrite_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.read_rule = ReadRule::Read;
    field.write_rule = WriteRule::Write;
}
