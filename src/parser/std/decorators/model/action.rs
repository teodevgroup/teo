use crate::core::model::builder::ModelBuilder;
use crate::parser::ast::argument::Argument;

pub(crate) fn action_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let pipeline = value.as_pipeline().unwrap();
    model.add_action_transformer(pipeline.clone());
}
