use std::sync::Arc;

use crate::{
    core::items::number::generation::{random_float::RandomFloatItem, random_int::RandomIntItem},
    parser::ast::argument::Argument,
};
use crate::core::item::Item;

pub(crate) fn random_int(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap()
        .as_value()
        .unwrap();
    Arc::new(RandomIntItem::new(value.clone()))
}

pub(crate) fn random_float(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap()
        .as_value()
        .unwrap();
    Arc::new(RandomFloatItem::new(value.clone()))
}
