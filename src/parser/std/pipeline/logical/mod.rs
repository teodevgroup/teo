use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::logical::valid::ValidModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn valid(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(ValidModifier::new())
}
