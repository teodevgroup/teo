use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::array::append::AppendModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn append(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AppendModifier::new(value))
}
