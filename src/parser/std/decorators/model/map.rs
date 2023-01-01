use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn map_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    model.table_name(args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap());
}
