use std::sync::Arc;
use crate::core::action::Action;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::action::when::WhenItem;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn when(args: Vec<Argument>) -> Arc<dyn Item> {
    let pipeline = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    match value {
        Value::RawOptionChoice(action_value) => {
            Arc::new(WhenItem::new(vec![Action::from_u32(*action_value)], pipeline.clone()))
        }
        Value::RawEnumChoice(enum_member) => {
            let action = Action::from_name(enum_member);
            Arc::new(WhenItem::new(vec![action], pipeline.clone()))
        }
        _ => {
            panic!()
        }
    }
}
