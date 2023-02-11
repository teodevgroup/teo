use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::object::get_object::GetObjectModifier;
use crate::core::pipeline::items::object::is_instance_of::IsObjectOfModifier;
use crate::core::pipeline::items::object::object_previous_value::ObjectPreviousValueModifier;
use crate::core::pipeline::items::object::object_set_value::ObjectSetValueModifier;
use crate::core::pipeline::items::object::object_value::ObjectValueModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn get_object(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(GetObjectModifier::new())
}

pub(crate) fn object_get(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectValueModifier::new(value))
}

pub(crate) fn object_set(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let value = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectSetValueModifier::new(key, value))
}

pub(crate) fn object_previous_value(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectPreviousValueModifier::new(key))
}

pub(crate) fn is_a(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( IsObjectOfModifier::new(key.as_raw_enum_choice().unwrap()))
}
