use crate::parser::ast::model::Model;

pub(crate) enum Reference<'a> {
    ModelReference(&'a Model),

}
