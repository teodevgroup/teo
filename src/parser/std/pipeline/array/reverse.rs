use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::array::reverse::ReverseModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn reverse(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ReverseModifier::new())
}
