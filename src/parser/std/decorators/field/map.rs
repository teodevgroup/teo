use crate::core::field::builder::FieldBuilder;


use crate::parser::ast::argument::Argument;

pub(crate) fn map_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    field.column_name(args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap());
}
