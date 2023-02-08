use crate::core::field::Field;
use crate::core::field::optionality::Optionality;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn present_without_decorator(args: Vec<Argument>, field: &mut Field) {
    let mut result: Vec<Value> = vec![];
    for arg in args.iter() {
        match arg.resolved.as_ref().unwrap().as_value().unwrap() {
            Value::RawEnumChoice(enum_choice) => {
                result.push(Value::RawEnumChoice(enum_choice.clone()))
            }
            Value::Vec(vec) => {
                result.push(Value::Vec(vec.clone()));
            }
            _ => panic!("Wrong argument passed to presentWithout.")
        }
    }
    field.optionality = Optionality::PresentWithout(result);
}
