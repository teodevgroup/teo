use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::items::array::append::AppendItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn append(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AppendItem::new(value))
}
