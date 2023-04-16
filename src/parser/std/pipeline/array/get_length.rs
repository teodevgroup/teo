use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::items::array::get_length::GetLengthItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn get_length(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(GetLengthItem::new())
}
