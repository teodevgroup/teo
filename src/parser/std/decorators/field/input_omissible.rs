use crate::core::field::builder::FieldBuilder;
use crate::parser::ast::argument::Argument;

pub(crate) fn input_ommissible_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.input_omissible();
}
