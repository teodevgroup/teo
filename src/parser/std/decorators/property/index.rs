use crate::core::property::Property;
use crate::parser::ast::argument::Argument;
use crate::parser::std::decorators::field::index::{decorator_for_index, FIELD_INDEX_UNIQUE, FIELD_INDEX_INDEX};

pub(crate) fn unique_decorator(args: &Vec<Argument>, property: &mut Property) {
    decorator_for_index(args, property, FIELD_INDEX_UNIQUE)
}

pub(crate) fn index_decorator(args: &Vec<Argument>, property: &mut Property) {
    decorator_for_index(args, property, FIELD_INDEX_INDEX)
}