use crate::core::field::field::Field;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn on_output_decorator(args: Vec<Argument>, field: &mut Field) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.on_output_pipeline = p.clone();
        }
        _ => panic!("Wrong argument passed to onOutput.")
    }
}
