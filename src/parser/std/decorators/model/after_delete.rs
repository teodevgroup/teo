use crate::core::model::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn after_delete_decorator(args: &Vec<Argument>, model: &mut Model) {
    model.set_after_delete_pipeline(args.unwrap().get(0).unwrap().get_value().unwrap().as_pipeline().unwrap().clone());
}
