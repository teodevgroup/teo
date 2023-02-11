use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::datetime::now::NowModifier;
use crate::core::pipeline::items::datetime::today::TodayModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn now(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(NowModifier::new())
}

pub(crate) fn today(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(TodayModifier::new())
}
