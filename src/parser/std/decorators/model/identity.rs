use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn identity_decorator(_args: Vec<Argument>, model: &mut ModelBuilder) {
    model.identity = true;
}
