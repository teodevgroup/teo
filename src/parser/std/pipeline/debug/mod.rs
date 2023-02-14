use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::debug::print::PrintItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn print(args: Vec<Argument>) -> Arc<dyn Item> {
    if args.len() == 0 {
        Arc::new(PrintItem::new(None))
    } else {
        Arc::new(PrintItem::new(Some(args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone())))
    }
}
