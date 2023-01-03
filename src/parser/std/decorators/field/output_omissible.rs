use crate::core::field::builder::FieldBuilder;
use crate::parser::ast::argument::Argument;

pub(crate) fn output_omissible_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.output_omissible();
}
