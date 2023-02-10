use crate::core::model::builder::ModelBuilder;
use crate::parser::ast::argument::Argument;

pub(crate) fn can_mutate_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    model.can_mutate_pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone();
}
