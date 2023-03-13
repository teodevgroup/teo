use crate::core::action::Action;
use crate::core::model::builder::ModelBuilder;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn disable_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let actions = match value {
        Value::RawOptionChoice(action_value) => {
            vec![Action::from_u32(*action_value)]
        }
        Value::RawEnumChoice(enum_member, _) => {
            let action = Action::from_name(enum_member);
            vec![action]
        }
        _ => {
            panic!()
        }
    };
    model.disabled_actions = Some(actions);
}
