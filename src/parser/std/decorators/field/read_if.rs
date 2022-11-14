use crate::core::field::Field;
use crate::core::field::read_rule::ReadRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn read_if_decorator(args: Vec<Argument>, field: &mut Field) {
    field.read_rule = ReadRule::ReadIf(args.get(0).unwrap().resolved.unwrap().as_pipeline().unwrap().clone());
}
