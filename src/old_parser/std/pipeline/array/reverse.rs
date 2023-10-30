use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::array::reverse::ReverseItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn reverse(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ReverseItem::new())
}
