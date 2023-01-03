use crate::core::field::builder::FieldBuilder;
use crate::core::field::Field;
use crate::core::field::write_rule::WriteRule;
use crate::parser::ast::argument::Argument;

pub(crate) fn write_nonnull_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.write_rule = WriteRule::WriteNonNull;
}
