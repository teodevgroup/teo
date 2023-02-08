use crate::core::field::Field;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn on_save_decorator(args: Vec<Argument>, field: &mut Field) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.on_save_pipeline = p.clone();
        }
        _ => panic!("Wrong argument passed to onSave.")
    }
}
