use crate::core::field::Field;
use crate::core::field::optionality::Optionality;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn present_if_decorator(args: Vec<Argument>, field: &mut Field) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::Pipeline(p) => {
            field.optionality = Optionality::PresentIf(p.clone());
        }
        _ => panic!("Wrong argument passed to presentIf.")
    }
}
