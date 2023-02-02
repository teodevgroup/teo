use crate::core::model::builder::ModelBuilder;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn unique_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match arg_value {
        Value::RawEnumChoice(str_value) => {
            model.unique(vec![str_value]);
        }
        Value::Vec(vec_value) => {
            let str_values: Vec<&str> = vec_value.iter().map(|v| v.as_raw_enum_choice().unwrap()).collect();
            model.unique(str_values);
        }
        _ => {
            panic!("Only string or array of string can be passed as @unique's argument.")
        }
    }
}
