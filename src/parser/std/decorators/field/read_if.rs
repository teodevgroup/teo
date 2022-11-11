use crate::core::field::Field;
use crate::core::field::read_rule::ReadRule;
use crate::core::field::write_rule::WriteRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn read_if_decorator(args: Vec<Argument>, field: &mut Field) {
    let arg = args.get(0).unwrap();
    field.write_rule = WriteRule::NoWrite;
}
