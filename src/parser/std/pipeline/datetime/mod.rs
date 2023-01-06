use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::datetime::now::NowModifier;
use crate::core::pipeline::modifiers::datetime::today::TodayModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn now(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(NowModifier::new())
}

pub(crate) fn today(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(TodayModifier::new())
}
