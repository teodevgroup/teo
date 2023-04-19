use crate::core::model::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn can_mutate_decorator(args: &Vec<Argument>, model: &mut Model) {
    model.set_can_mutate_pipeline(args.unwrap().get(0).unwrap().get_value().unwrap().as_pipeline().unwrap().clone());
}
