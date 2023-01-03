use crate::core::field::builder::FieldBuilder;
use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn virtual_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.r#virtual();
}
