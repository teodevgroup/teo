use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::array::reverse::ReverseModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn reverse(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(ReverseModifier::new())
}
