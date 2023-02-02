use crate::core::model::builder::ModelBuilder;

use crate::parser::ast::argument::Argument;

pub(crate) fn virtual_decorator(_args: Vec<Argument>, model: &mut ModelBuilder) {
    model.r#virtual = true;
}
