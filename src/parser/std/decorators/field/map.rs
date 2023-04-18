use crate::core::field::field::Field;


use crate::parser::ast::argument::Argument;

pub(crate) fn map_decorator(args: Vec<Argument>, field: &mut Field) {
    field.column_name = Some(args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_string());
}
