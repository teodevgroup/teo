use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::number::is_even::IsEvenModifier;
use crate::core::pipeline::items::number::is_odd::IsOddModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn is_even(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsEvenModifier::new())
}

pub(crate) fn is_odd(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsOddModifier::new())
}
