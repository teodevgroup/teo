use std::sync::{Arc, Mutex};
use crate::core::app::builder::CallbackLookupTable;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::array::get_length::GetLengthModifier;
use crate::core::pipeline::modifiers::bcrypt::bcrypt_salt::BcryptSaltModifier;
use crate::core::pipeline::modifiers::bcrypt::bcrypt_verify::BcryptVerifyModifier;
use crate::core::pipeline::modifiers::logical::transform_with::TransformWithModifier;
use crate::core::pipeline::modifiers::logical::validate_with::ValidateWithModifier;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn custom_transform(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Modifier> {
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
            Arc::new(TransformWithModifier::new(p.clone()))
        }
        _ => panic!("Argument to `transform` should be string or pipeline.")
    }
}

pub(crate) fn custom_callback(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Modifier> {
    let name = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    let lookup_table = lookup_table.lock().unwrap();
    let modifier = lookup_table.callbacks.get(name);
    if let Some(modifier) = modifier {
        modifier.clone()
    } else {
        panic!("Cannot find a callback named '{}'.", name)
    }
}

pub(crate) fn custom_validate(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Modifier> {
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
            Arc::new(ValidateWithModifier::new(p.clone()))
        }
        _ => panic!("Argument to `validate` should be string or pipeline.")
    }
}

pub(crate) fn custom_compare(lookup_table: Arc<Mutex<CallbackLookupTable>>, args: Vec<Argument>) -> Arc<dyn Modifier> {
    let name = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    let lookup_table = lookup_table.lock().unwrap();
    let modifier = lookup_table.compares.get(name);
    if let Some(modifier) = modifier {
        modifier.clone()
    } else {
        panic!("Cannot find a compare named '{}'.", name)
    }
}
