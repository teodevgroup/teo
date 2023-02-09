use std::sync::Arc;
use crate::core::action::Action;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::action::when::WhenModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn when(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let action = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_raw_option_choice().unwrap();
    let pipeline = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    Arc::new(WhenModifier::new(vec![Action::from_u32(action)], pipeline.clone()))
}
