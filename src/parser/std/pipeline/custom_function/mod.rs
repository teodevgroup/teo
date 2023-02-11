use std::sync::{Arc, Mutex};
use crate::core::app::builder::CallbackLookupTable;
use crate::core::pipeline::item::Item;



use crate::core::pipeline::items::logical::transform_with::TransformWithItem;
use crate::core::pipeline::items::logical::validate_with::ValidateWithItem;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn custom_transform(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Item> {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match arg_value {
        Value::String(s) => {
            let name = s.as_str();
            let lookup_table = lookup_table.lock().unwrap();
            let modifier = lookup_table.transforms.get(name);
            if let Some(modifier) = modifier {
                modifier.clone()
            } else {
                panic!("Cannot find a transform named '{}'.", name)
            }
        }
        Value::Pipeline(p) => {
            Arc::new(TransformWithItem::new(p.clone()))
        }
        _ => panic!("Argument to `transform` should be string or pipeline.")
    }
}

pub(crate) fn custom_callback(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Item> {
    let name = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    let lookup_table = lookup_table.lock().unwrap();
    let modifier = lookup_table.callbacks.get(name);
    if let Some(modifier) = modifier {
        modifier.clone()
    } else {
        panic!("Cannot find a callback named '{}'.", name)
    }
}

pub(crate) fn custom_validate(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Item> {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match arg_value {
        Value::String(s) => {
            let name = s.as_str();
            let lookup_table = lookup_table.lock().unwrap();
            let modifier = lookup_table.validators.get(name);
            if let Some(modifier) = modifier {
                modifier.clone()
            } else {
                panic!("Cannot find a validate named '{}'.", name)
            }
        }
        Value::Pipeline(p) => {
            Arc::new(ValidateWithItem::new(p.clone()))
        }
        _ => panic!("Argument to `validate` should be string or pipeline.")
    }
}

pub(crate) fn custom_compare(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Item> {
    let name = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    let lookup_table = lookup_table.lock().unwrap();
    let modifier = lookup_table.compares.get(name);
    if let Some(modifier) = modifier {
        modifier.clone()
    } else {
        panic!("Cannot find a compare named '{}'.", name)
    }
}
