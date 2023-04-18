use crate::core::field::field::Sort;
use crate::core::model::model::Model;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

static MODEL_INDEX_PRIMARY: u8 = 0;
static MODEL_INDEX_INDEX: u8 = 1;
static MODEL_INDEX_UNIQUE: u8 = 2;

pub(crate) fn id_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    decorator(args, model, MODEL_INDEX_PRIMARY)
}

pub(crate) fn index_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    decorator(args, model, MODEL_INDEX_INDEX)
}

pub(crate) fn unique_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    decorator(args, model, MODEL_INDEX_UNIQUE)
}

fn decorator(args: Option<&Vec<Argument>>, model: &mut Model, index_kind: u8) {
    let args = args.unwrap();
    let mut items: Vec<ModelIndexItem> = vec![];
    let mut map: Option<String> = None;
    if args.is_empty() {
        panic!("Model index decorator takes at least one argument.")
    }
    let arg0 = args.get(0).unwrap();
    if arg0.name.is_some() && (arg0.name.as_ref().unwrap().name.as_str() != "fields") {
        panic!("Model index decorator's first argument should be fields or no name.")
    }
    let arg0_value = arg0.resolved.as_ref().unwrap().as_value().unwrap();
    match arg0_value {
        Value::Vec(vec) => {
            for value in vec {
                match value {
                    Value::RawEnumChoice(name, args) => {
                        items.push(model_index_item(name, args));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Value::RawEnumChoice(name, args) => {
            items.push(model_index_item(name, args));
        }
        _ => unreachable!(),
    }
    // map name
    if let Some(arg1) = args.get(1) {
        if arg1.name.is_none() || (arg1.name.as_ref().unwrap().name.as_str() != "map") {
            panic!("Model index decorator's second argument should be map.")
        }
        map = Some(arg1.resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_owned());
    }
    match index_kind {
        0 => model.add_index(ModelIndex::new(ModelIndexType::Primary, map, items)),
        1 => model.add_index(ModelIndex::new(ModelIndexType::Index, map, items)),
        2 => model.add_index(ModelIndex::new(ModelIndexType::Unique, map, items)),
        _ => unreachable!(),
    }
}

fn model_index_item(name: &str, args: &Option<Vec<(Option<String>, Value)>>) -> ModelIndexItem {
    let mut sort = Sort::Asc;
    let mut len: Option<usize> = None;
    if let Some(args) = args {
        for (index, (arg_name, arg_value)) in args.iter().enumerate() {
            match index {
                0 => {
                    if arg_name.is_some() && (arg_name.as_ref().unwrap().as_str() != "fields") {
                        panic!("Unknown argument {}", arg_name.as_ref().unwrap());
                    }
                    match arg_value.as_raw_enum_choice().unwrap() {
                        "asc" => sort = Sort::Asc,
                        "desc" => sort = Sort::Desc,
                        _ => unreachable!(),
                    }
                }
                1 => {
                    if arg_name.is_none() || (arg_name.as_ref().unwrap().as_str() != "length") {
                        panic!("Second argument should be length.");
                    }
                    len = Some(arg_value.as_usize().unwrap());
                }
                _ => unreachable!(),
            }
        }
    }
    ModelIndexItem::new(name, sort, len)
}
