use crate::core::property::Property;
use crate::parser::ast::argument::Argument;

pub(crate) fn cached_decorator(_args: &Vec<Argument>, property: &mut Property) {
    property.cached = true;
}
