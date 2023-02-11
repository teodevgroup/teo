use std::sync::Arc;
use crate::core::pipeline::item::Item;


use crate::core::pipeline::items::array::truncate::TruncateModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn truncate(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(TruncateModifier::new(value.clone()))
}
