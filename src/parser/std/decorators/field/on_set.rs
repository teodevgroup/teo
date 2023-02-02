use crate::core::field::builder::FieldBuilder;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn on_set_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.on_set(p.clone());
        }
        _ => panic!("Wrong argument passed to onSet.")
    }
}
