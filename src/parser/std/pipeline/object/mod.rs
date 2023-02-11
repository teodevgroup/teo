use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::object::get_object::GetObjectItem;
use crate::core::pipeline::items::object::is_instance_of::IsObjectOfItem;
use crate::core::pipeline::items::object::object_previous_value::ObjectPreviousValueItem;
use crate::core::pipeline::items::object::object_set_value::ObjectSetValueItem;
use crate::core::pipeline::items::object::object_value::ObjectValueItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn get_object(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(GetObjectItem::new())
}

pub(crate) fn object_get(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectValueItem::new(value))
}

pub(crate) fn object_set(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let value = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectSetValueItem::new(key, value))
}

pub(crate) fn object_previous_value(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( ObjectPreviousValueItem::new(key))
}

pub(crate) fn is_a(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( IsObjectOfItem::new(key.as_raw_enum_choice().unwrap()))
}
