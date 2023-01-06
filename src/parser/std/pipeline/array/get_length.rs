use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::array::get_length::GetLengthModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn get_length(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(GetLengthModifier::new())
}
