use crate::core::property::builder::PropertyBuilder;

use crate::parser::ast::argument::Argument;

pub(crate) fn cached_decorator(_args: Vec<Argument>, property: &mut PropertyBuilder) {
    property.cached = true;
}
