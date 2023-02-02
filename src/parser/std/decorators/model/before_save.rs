use crate::core::model::builder::ModelBuilder;
use crate::parser::ast::argument::Argument;

pub(crate) fn before_save_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    model.before_save_pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone();
}
