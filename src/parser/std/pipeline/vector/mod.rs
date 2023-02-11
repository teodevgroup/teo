use std::sync::Arc;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::items::vector::filter::FilterModifier;
use crate::core::pipeline::items::vector::item_at::ItemAtModifier;
use crate::core::pipeline::items::vector::join::JoinModifier;
use crate::core::pipeline::items::vector::map::MapModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn join(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(JoinModifier::new(value.clone()))
}

pub(crate) fn item_at(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(ItemAtModifier::new(value))
}

pub(crate) fn filter(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(FilterModifier::new(value.as_pipeline().unwrap().clone()))
}

pub(crate) fn map(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(MapModifier::new(value.as_pipeline().unwrap().clone()))
}
