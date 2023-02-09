use std::sync::Arc;
use crate::core::action::Action;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::action::when::WhenModifier;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn when(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let pipeline = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match value {
        Value::RawOptionChoice(action_value) => {
            Arc::new(WhenModifier::new(vec![Action::from_u32(*action_value)], pipeline.clone()))
        }
        Value::RawEnumChoice(enum_member) => {
            let action = Action::from_name(enum_member);
            Arc::new(WhenModifier::new(vec![action], pipeline.clone()))
        }
        _ => {
            panic!()
        }
    }
}
