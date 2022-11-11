use crate::core::property::Property;
use crate::parser::ast::argument::Argument;

pub(crate) trait PropertyDecorator {
    fn decorate(&self, args: Vec<Argument>, property: &mut Property);
}
