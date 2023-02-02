use crate::core::field::builder::FieldBuilder;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn on_save_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.on_save(p.clone());
        }
        _ => panic!("Wrong argument passed to onSave.")
    }
}
