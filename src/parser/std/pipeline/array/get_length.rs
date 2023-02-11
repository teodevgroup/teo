use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::array::get_length::GetLengthModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn get_length(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(GetLengthModifier::new())
}
