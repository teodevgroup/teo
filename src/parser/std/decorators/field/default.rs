use crate::core::field::field::Field;
use crate::parser::ast::argument::Argument;
use crate::parser::ast::entity::Entity;

pub(crate) fn default_decorator(args: Vec<Argument>, field: &mut Field) {
    match args.get(0).unwrap().resolved.as_ref().unwrap() {
        Entity::Value(value) => {
            field.default = Some(value.clone());
            field.input_omissible = true;
        }
        _ => {
            panic!("Only value default is supported for now.")
        }
    }
}
