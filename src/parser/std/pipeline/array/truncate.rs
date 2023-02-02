use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;


use crate::core::pipeline::modifiers::array::truncate::TruncateModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn truncate(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(TruncateModifier::new(value.clone()))
}
