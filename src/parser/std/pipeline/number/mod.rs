use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::number::is_even::IsEvenModifier;
use crate::core::pipeline::modifiers::number::is_odd::IsOddModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn is_even(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(IsEvenModifier::new())
}

pub(crate) fn is_odd(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(IsOddModifier::new())
}
