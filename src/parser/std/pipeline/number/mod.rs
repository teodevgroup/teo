use crate::core::pipeline::item::Item;
use crate::core::items::number::is_even::IsEvenItem;
use crate::core::items::number::is_odd::IsOddItem;
use crate::parser::ast::argument::Argument;
use std::sync::Arc;

pub(crate) mod generation;

pub(crate) fn is_even(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsEvenItem::new())
}

pub(crate) fn is_odd(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsOddItem::new())
}
