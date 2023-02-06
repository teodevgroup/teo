use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::debug::print::PrintModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn print(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(PrintModifier::new())
}
