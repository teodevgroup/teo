use crate::core::field::builder::FieldBuilder;
use crate::core::field::Field;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn auth_by_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.auth_by_arg = Some(Value::Pipeline(p.clone()));
        }
        _ => panic!("Wrong argument passed to authBy.")
    }
}
