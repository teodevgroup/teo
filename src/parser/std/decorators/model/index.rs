use crate::core::model::builder::ModelBuilder;

use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

static VALID_NAMES: [&str; 1] = ["map"];

static FIELD_INDEX_PRIMARY: u8 = 0;
static FIELD_INDEX_INDEX: u8 = 1;
static FIELD_INDEX_UNIQUE: u8 = 2;

pub(crate) fn id_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match arg_value {
        Value::String(str_value) => {
            model.primary(vec![str_value]);
        }
        Value::Vec(vec_value) => {
            let str_values: Vec<&str> = vec_value.iter().map(|v| v.as_raw_enum_choice().unwrap()).collect();
            model.primary(str_values);
        }
        _ => {
            panic!("Only string or array of string can be passed as @id's argument.")
        }
    }
}

pub(crate) fn index_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match arg_value {
        Value::RawEnumChoice(str_value) => {
            model.index(vec![str_value]);
        }
        Value::Vec(vec_value) => {
            let str_values: Vec<&str> = vec_value.iter().map(|v| v.as_raw_enum_choice().unwrap()).collect();
            model.index(str_values);
        }
        _ => {
            panic!("Only string or array of string can be passed as @index's argument.")
        }
    }
}

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

fn decorator(args: Vec<Argument>, model: &mut ModelBuilder, index_kind: u8) {
    let mut map: Option<String> = None;
    if args.is_empty() {
        panic!("Model index decorator takes at least one argument.")
    }
    let arg0 = args.get(0).unwrap();
    if arg0.name.is_some() && (arg0.name.as_ref().unwrap().name.as_str() != "fields") {
        panic!("Model index decorator's first argument should be fields or no name.")
    }
    // map name
    if let Some(arg1) = args.get(1) {
        if arg1.name.is_none() || (arg1.name.as_ref().unwrap().name.as_str() != "map") {
            panic!("Model index decorator's second argument should be map.")
        }
        map = Some(arg1.resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_owned());
    }
}
