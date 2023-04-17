use crate::core::model::model::Model;
use crate::parser::ast::argument::Argument;

pub(crate) fn after_save_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    model.set_after_save_pipeline(args.unwrap().get(0).unwrap().get_value().unwrap().as_pipeline().unwrap().clone());
}
