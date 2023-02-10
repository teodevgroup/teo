use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::identity::identity::IdentityModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn identity(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    Arc::new(IdentityModifier::new(pipeline.clone()))
}
