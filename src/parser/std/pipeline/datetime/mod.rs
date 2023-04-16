use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::datetime::now::NowItem;
use crate::core::items::datetime::today::TodayItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn now(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(NowItem::new())
}

pub(crate) fn today(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(TodayItem::new())
}
