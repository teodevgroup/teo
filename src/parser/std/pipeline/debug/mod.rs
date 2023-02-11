use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::debug::print::PrintItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn print(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(PrintItem::new())
}
