use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::items::object::assign::AssignItem;
use crate::core::items::object::ctx_self::SelfItem;
use crate::core::items::object::is_object_of::IsObjectOfItem;
use crate::core::items::object::get_previous::GetPreviousItem;
use crate::core::items::object::set::SetItem;
use crate::core::items::object::get::GetItem;
use crate::core::items::object::is::IsItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn ctx_self(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(SelfItem::new())
}

pub(crate) fn object_get(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( GetItem::new(value.clone()))
}

pub(crate) fn object_set(args: Vec<Argument>) -> Arc<dyn Item> {
    match args.len() {
        1 => {
            let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
            Arc::new( SetItem::new(None, value.clone()))
        }
        2 => {
            let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
            let value = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
            Arc::new( SetItem::new(Some(key.clone()), value.clone()))
        }
        _ => panic!("wrong number of arguments to set")
    }
}

pub(crate) fn object_previous_value(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( GetPreviousItem::new(key))
}

pub(crate) fn is_a(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( IsObjectOfItem::new(key.as_raw_enum_choice().unwrap()))
}

pub(crate) fn is(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let relation_name = match args.get(1) {
        Some(arg) => Some(arg.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_string()),
        None => None,
    };
    Arc::new( IsItem::new(value.clone(), relation_name))
}

pub(crate) fn assign(args: Vec<Argument>) -> Arc<dyn Item> {
    let key = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let value = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AssignItem::new(key.clone(), value.clone()))
}
