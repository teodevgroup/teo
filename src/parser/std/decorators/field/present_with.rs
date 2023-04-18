use crate::core::field::field::Field;
use crate::core::field::optionality::Optionality;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn present_with_decorator(args: Vec<Argument>, field: &mut Field) {
    let mut result: Vec<Value> = vec![];
    for arg in args.iter() {
        match arg.resolved.as_ref().unwrap().as_value().unwrap() {
            Value::RawEnumChoice(enum_choice, _) => {
                result.push(Value::RawEnumChoice(enum_choice.clone(), None))
            }
            Value::Vec(vec) => {
                result.push(Value::Vec(vec.clone()));
            }
            _ => panic!("Wrong argument passed to presentWith.")
        }
    }
    field.optionality = Optionality::PresentWith(result);
}
