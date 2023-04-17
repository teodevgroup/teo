use crate::core::model::model::Model;

use crate::parser::ast::argument::Argument;

pub(crate) fn identity_decorator(_args: Option<&Vec<Argument>>, model: &mut Model) {
    model.set_identity(true);
}
