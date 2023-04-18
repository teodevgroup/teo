use crate::core::field::field::Field;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn auth_by_decorator(args: Vec<Argument>, field: &mut Field) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.identity_checker = Some(Value::Pipeline(p.clone()));
        }
        _ => panic!("Wrong argument passed to authBy.")
    }
}
