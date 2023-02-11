use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::datetime::now::NowItem;
use crate::core::pipeline::items::datetime::today::TodayItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn now(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(NowItem::new())
}

pub(crate) fn today(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(TodayItem::new())
}
