use crate::core::model::model::Model;

use crate::parser::ast::argument::Argument;

pub(crate) fn virtual_decorator(_args: &Vec<Argument>, model: &mut Model) {
    model.set_virtual(true);
}
