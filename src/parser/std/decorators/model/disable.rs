use crate::core::action::Action;
use crate::core::model::model::Model;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn disable_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    let value = args.unwrap().get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
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
    model.set_disabled_actions(actions);
}
