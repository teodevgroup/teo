use crate::core::field::builder::FieldBuilder;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn present_without_decorator(args: Vec<Argument>, field: &mut FieldBuilder) {
    match args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap() {
        Value::RawEnumChoice(enum_choice) => {
            field.present_without(vec![enum_choice]);
        }
        Value::Vec(enum_choices) => {
            let str_choices: Vec<&str> = enum_choices.iter().map(|c| {
                c.as_raw_enum_choice().unwrap()
            }).collect();
            field.present_without(str_choices);
        }
        _ => panic!("Wrong argument passed to presentWithout.")
    }
}
