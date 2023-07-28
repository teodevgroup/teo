use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::datetime::now::NowItem;
use crate::core::items::datetime::today::TodayItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn now(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(NowItem::new())
}

pub(crate) fn today(args: &Vec<Argument>) -> Arc<dyn Item> {
    let timezone = match args.get(0) {
        Some(arg) => {
            arg.resolved.as_ref().unwrap().as_value().unwrap().as_i32().unwrap()
        }
        None => 0
    };
    Arc::new(TodayItem::new(timezone))
}
