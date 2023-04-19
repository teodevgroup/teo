use crate::core::model::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn can_read_decorator(args: &Vec<Argument>, model: &mut Model) {
    model.set_can_read_pipeline(args.get(0).unwrap().get_value().unwrap().as_pipeline().unwrap().clone());
}
