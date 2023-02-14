use std::sync::Arc;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::items::vector::filter::FilterItem;
use crate::core::pipeline::items::vector::item_at::AtItem;
use crate::core::pipeline::items::vector::join::JoinItem;
use crate::core::pipeline::items::vector::map::MapItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn join(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(JoinItem::new(value.clone()))
}

pub(crate) fn item_at(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AtItem::new(value))
}

pub(crate) fn filter(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(FilterItem::new(value.as_pipeline().unwrap().clone()))
}

pub(crate) fn map(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(MapItem::new(value.as_pipeline().unwrap().clone()))
}
