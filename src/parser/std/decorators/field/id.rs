use crate::core::field::builder::FieldBuilder;
use crate::core::field::Field;
use crate::parser::ast::accessible::FieldDecorator;
use crate::parser::ast::argument::Argument;

pub(crate) fn id_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.primary = true;
}
