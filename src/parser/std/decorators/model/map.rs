use crate::core::model::model::Model;

use crate::parser::ast::argument::Argument;

pub(crate) fn map_decorator(args: &'static Vec<Argument>, model: &mut Model) {
    model.set_table_name(args.get(0).unwrap().get_value().unwrap().as_str().unwrap());
}
